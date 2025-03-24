use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use argon2::password_hash::rand_core::{OsRng, RngCore};
use rocket::{
    State,
    futures::{SinkExt, StreamExt},
    get,
    serde::Serialize,
    tokio::{
        select,
        sync::{
            RwLock,
            broadcast::{self, Sender},
            watch,
        },
    },
};
use rocket_ws::{self as ws, WebSocket, stream::DuplexStream};
use serde_json::{Number, Value, json};
use sqlx::PgPool;

use crate::{
    hvahoot::{Question, get_data},
    login::User,
};

#[get("/play/<game_id>")]
pub async fn play(
    game_id: u32,
    ws: WebSocket,
    games: &State<Arc<RwLock<HashMap<u32, Game>>>>,
) -> Option<ws::Channel> {
    eprintln!("{}: {}", file!(), line!());
    let sender = games
        .write()
        .await
        .get_mut(&game_id)
        .map(|x| x.sender.clone())?;
    let mut receiver = games
        .write()
        .await
        .get_mut(&game_id)
        .map(|x| x.game_state.clone())?;
    let client_id = games.write().await.get_mut(&game_id).map(|x| {
        x.counter += 1;
        x.counter
    })?;
    let mut answered = 0;
    let mut next_correct = 0;
    let mut points = 0;
    Some(ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    message = stream.next() => {
                        let Some(message) = message.transpose()? else {
                        sender.send(Protocol::Disconnected{ client_id }).unwrap();
                            break;
                        };
                        let raw = message.to_text()?;
                        let Ok(Value::Object(obj)) = serde_json::from_str(raw) else {
                            return Ok(());
                        };
                        let Some(r#type) = obj.get("type") else {
                            return Ok(());
                        };
                        match r#type {
                            Value::String(x) if x == "username" => {
                                let Some(Value::String(username)) = obj.get("username") else {
                                    return Ok(());
                                };
                                sender.send(Protocol::Connected{client_id, username: username.clone()}).unwrap();
                            }
                            Value::String(x) if x == "answer" => {
                                let Some(Value::Number(x)) = obj.get("answer") else {
                                    return Ok(());
                                };
                                let Some(num) = x.as_i64() else {
                                    return Ok(());
                                };
                                answered = num as i32;
                                if answered == next_correct {
                                    points += 1;
                                }
                                let _ = sender.send(Protocol::Answer { client_id, answer: answered });
                            }
                            _ => return Ok(()),
                        }
                    },
                    update_question = receiver.changed() => {
                        if update_question.is_err()  {
                            return Ok(())
                        };
                        let binding = games.read().await;
                        let x: GameState = receiver.borrow().clone();
                        match x {
                            GameState::Question(question_num) => {
                                let q = &binding.get(&game_id).unwrap().questions[question_num];
                                next_correct = q.answer;
                                let _ = stream.send_json(
                                    &json!({
                                        "type": "answer",
                                        "question": q.question,
                                        "answers": q.answers,
                                    })).await;
                            }
                            GameState::ShowResult(question_num) => {
                                let q = &binding.get(&game_id).unwrap().questions[question_num];
                                let _ = stream.send_json(
                                    &json!({
                                        "type": "question",
                                        "correct": answered == q.answer,
                                        "points": points,
                                    })).await;
                            }
                            GameState::Pending => unreachable!(),
                        }
                    }
                }
            }

            Ok(())
        })
    }))
}

#[derive(Clone, Debug)]
pub enum Protocol {
    Connected { client_id: usize, username: String },
    Disconnected { client_id: usize },
    Answer { client_id: usize, answer: i32 },
}

#[derive(Debug, Clone)]
pub struct Game {
    quiz_uuid: String,
    curr: usize,
    questions: Vec<Question>,
    sender: Sender<Protocol>,
    game_state: watch::Receiver<GameState>,
    counter: usize,
}

#[derive(Debug, Clone)]
enum GameState {
    Question(usize),
    ShowResult(usize),
    Pending,
}

#[get("/play/host/<uuid>")]
pub async fn host<'a>(
    uuid: String,
    ws: WebSocket,
    user: User,
    games: &'a State<Arc<RwLock<HashMap<u32, Game>>>>,
    pool: &State<PgPool>,
) -> Option<ws::Channel<'a>> {
    let (sender, mut recv) = broadcast::channel(1);
    let (curr_sender, curr_receiver) = watch::channel(GameState::Pending);
    let id = OsRng::next_u32(&mut OsRng) & !2u32.pow(31);
    let questions = get_data(pool, &user, &uuid).await?.questions;
    let mut curr = 0;
    games.write().await.insert(
        id,
        Game {
            sender,
            game_state: curr_receiver,
            curr: 0,
            questions: questions.clone(),
            quiz_uuid: uuid,
            counter: 0,
        },
    );
    Some(ws.channel(move |mut stream| {
        Box::pin(async move {
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&serde_json::json!({
                        "type": "code",
                        "code": id,
                    }))
                    .unwrap(),
                ))
                .await;

            let mut players = HashMap::<_, Player>::new();
            let mut answered = HashSet::new();
            'game_loop: loop {
                select! {
                    message = stream.next() => {
                        let Some(Ok(message)) = message else {
                            break;
                        };
                        let Ok(raw) = message.to_text() else {break;};
                        let Ok(Value::Object(obj)) = serde_json::from_str(raw) else {
                            break;
                        };
                        let Some(r#type) = obj.get("type") else {
                            return Ok(());
                        };

                        match r#type {
                            Value::String(x) if x == "next" => {
                                let mut binding = games.write().await;
                                let x = binding.get_mut(&id).unwrap();
                                let _ = curr_sender.send(GameState::Question(x.curr));
                                answered.clear();
                                stream.send_json(json!({
                                    "type": "question",
                                    "question": questions[x.curr].question,
                                    "answers": questions[x.curr].answers,
                                })).await;
                                x.curr += 1;
                                curr += 1;
                                if curr == questions.len() {
                                    break 'game_loop;
                                }
                            }
                            _ => return Ok(()),
                        }
                    },
                    message = recv.recv() => {
                        let Ok(message) = message else {
                            break;
                        };
                        match message {
                            Protocol::Connected {
                                client_id,
                                username,
                            } => {
                                println!("{client_id} joined with {username}");
                                stream.send_json(json!({
                                    "type": "join",
                                    "username": &username,
                                    "id": client_id
                                })).await;
                                players.insert(client_id, Player { name: username, client_id, score: 0 });
                            },
                            Protocol::Disconnected { client_id } => {
                                if let Some(Player {name: old_name, client_id, ..}) = players.remove(&client_id) {
                                    stream.send_json(json!({
                                        "type": "disconnect",
                                        "username": old_name,
                                        "id": client_id,
                                    })).await;
                                }
                                println!("{client_id} left");
                            },
                            Protocol::Answer{client_id, answer} => {
                                eprintln!("{client_id} answered {answer}");
                                stream.send_json(json!({
                                    "type": "answer",
                                    "id": client_id,
                                })).await;
                                if answered.insert(client_id) && answer == questions[curr].answer {
                                    if let Some(player) = players.get_mut(&client_id) {
                                        player.score += 1;
                                    }
                                };
                            }
                        };
                        if players.keys().copied().collect::<HashSet<_>>() == answered {
                            let _ = curr_sender.send(GameState::ShowResult(curr));
                            stream.send_json(json!({
                                "type": "summary",
                                "scores": players,
                            })).await;
                        }
                    },

                }
            }
            Ok(())
        })
    }))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Player {
    name: String,
    client_id: usize,
    score: usize,
}

#[get("/path")]
pub fn check(_u: User) {}

trait SendJson {
    fn send_json<T>(&mut self, data: T) -> impl Future<Output = ()>
    where
        T: Serialize;
}

impl SendJson for DuplexStream {
    async fn send_json<T>(&mut self, data: T)
    where
        T: Serialize,
    {
        let _ = self
            .send(ws::Message::Text(serde_json::to_string(&data).unwrap()))
            .await;
    }
}
