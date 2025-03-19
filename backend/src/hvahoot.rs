use rocket::{State, get, serde::json::Json};
use sqlx::{Acquire, PgPool};
use uuid::Uuid;

use crate::login::User;

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
