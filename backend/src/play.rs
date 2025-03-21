use std::{collections::HashMap, sync::Arc};

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
            broadcast::{self, Receiver, Sender},
        },
    },
};
use rocket_ws::{self as ws, WebSocket};
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    hvahoot::{Question, get_questions},
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
        .map(|x| x.switch_game.resubscribe())?;
    Some(ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    message = stream.next() => {
                        let Some(message) = message.transpose()? else {
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
                            }
                            Value::String(x) if x == "answer" => {
                                todo!()
                            }
                            _ => return Ok(()),
                        }
                        sender.send(Protocol::Connected).unwrap();
                    },
                    update_question = receiver.recv() => {
                        let binding = games.read().await;
                        let game = binding.get(&game_id).unwrap();
                        let _ = stream.send(ws::Message::Text(serde_json::to_string(&game.questions[game.curr]).unwrap())).await;
                    }
                }
            }

            Ok(())
        })
    }))
}

#[derive(Clone, Copy, Debug)]
pub enum Protocol {
    Connected,
    Disconnected,
    Answer(u32),
}

pub struct Game {
    quiz_uuid: String,
    curr: usize,
    questions: Vec<Question>,
    sender: Sender<Protocol>,
    switch_game: Receiver<()>,
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
    let (curr_sender, curr_receiver) = broadcast::channel(1);
    let id = OsRng::next_u32(&mut OsRng) & !2u32.pow(31);
    games.write().await.insert(
        id,
        Game {
            sender,
            switch_game: curr_receiver,
            curr: 0,
            questions: get_questions(pool, &user, &uuid).await?,
            quiz_uuid: uuid,
        },
    );
    Some(ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
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
                                games.write().await.get_mut(&id).unwrap().curr += 1;
                                let _ = curr_sender.send(());
                            }
                            _ => return Ok(()),
                        }
                        println!("{:?}", message)
                    },
                    message = recv.recv() => {
                        if let Ok(message) = message {
                            handle_message(message).await;
                        } else {
                            break;
                        }
                    },

                }
            }
            Ok(())
        })
    }))
}

async fn handle_message(message: Protocol) {
    match message {
        Protocol::Connected => eprintln!("Connected"),
        Protocol::Disconnected => eprintln!("Disconnected"),
        Protocol::Answer(x) => eprintln!("answered {x}"),
    }
}

#[get("/path")]
pub fn check(_u: User) {}
