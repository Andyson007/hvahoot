use rocket::{
    State, get, post,
    serde::{Deserialize, json::Json},
};
use sqlx::{Acquire, PgPool};
use uuid::Uuid;

use crate::login::User;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HvaHootData {
    name: String,
    questions: Vec<Question>,
    uuid: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    question: String,
    answer: i32,
    answers: [String; 4],
}

#[post("/api/create", data = "<hvahoot>")]
pub async fn create_hvahoot(
    user: User,
    hvahoot: Json<HvaHootData>,
    pool: &State<PgPool>,
) -> Option<()> {
    let Ok(mut pool) = pool.acquire().await else {
        eprintln!("failed to establish connection");
        return None;
    };
    let hvahoot_id = sqlx::query!(
        "UPDATE hvahoots SET name=$1 WHERE uuid=$2 AND owner=$3 RETURNING id",
        hvahoot.name,
        hvahoot.uuid,
        user.id,
    )
    .fetch_optional(pool.acquire().await.ok()?)
    .await
    .ok()?
    .map(|x| x.id)?;

    sqlx::query!("DELETE FROM questions WHERE hvahoot=$1", hvahoot_id)
        .fetch_optional(pool.acquire().await.ok()?)
        .await
        .ok()?;

    for question in &hvahoot.questions {
        if let Err(e) = sqlx::query!(
            "INSERT INTO questions (hvahoot, question, answers, correct) VALUES ($1, $2, $3, $4)",
            hvahoot_id,
            question.question,
            &question.answers,
            question.answer,
        )
        .execute(pool.acquire().await.ok()?)
        .await
        {
            println!("Error occured while inserting the questions: {e}")
        }
    }
    Some(())
}

#[get("/api/create/get")]
pub async fn get_uuid(user: User, pool: &State<PgPool>) -> Option<String> {
    let uuid = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO hvahoots (owner, uuid) VALUES ($1, $2)",
        user.id,
        uuid.to_string()
    )
    .execute(
        pool.acquire()
            .await
            .map_err(|e| {
                println!("{e}");
            })
            .ok()?
            .acquire()
            .await
            .map_err(|e| println!("{e}"))
            .ok()?,
    )
    .await
    .map_err(|e| println!("{e}"))
    .ok()?;

    Some(uuid.to_string())
}
