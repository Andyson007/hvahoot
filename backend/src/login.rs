use std::{
    env, fs,
    path::{Path, PathBuf},
};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use rocket::{
    State,
    futures::TryFutureExt,
    http::CookieJar,
    post,
    serde::{Deserialize, json::Json},
};
use sqlx::{Acquire, Executor, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(
    login: Json<Login>,
    cookies: &CookieJar<'_>,
    pool: &State<PgPool>,
) -> Result<(), ()> {
    let login = login.0;
    let (id, phc) = sqlx::query!(
        "SELECT id, phc FROM clients WHERE username=$1 LIMIT 1",
        login.username
    )
    .fetch_one(
        pool.acquire()
            .await
            .map_err(|e| {
                println!("{e}");
            })?
            .acquire()
            .await
            .map_err(|e| println!("{e}"))?,
    )
    .map_ok(|x| (x.id, x.phc))
    .await
    .map_err(|e| eprintln!("{e}"))?;
    let argon = Argon2::default();
    if argon
        .verify_password(
            login.password.as_bytes(),
            &PasswordHash::new(&phc).map_err(|e| println!("{e}"))?,
        )
        .is_ok()
    {
        let token = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO tokens (token, client) VALUES ($1, $2)",
            &token.to_string(),
            id
        )
        .execute(
            pool.acquire()
                .await
                .map_err(|e| {
                    println!("{e}");
                })?
                .acquire()
                .await
                .map_err(|e| println!("{e}"))?,
        )
        .await
        .map_err(|e| println!("{e}"))?;
        cookies.add(("token", token.to_string()));
        Ok(())
    } else {
        Err(())
    }
}

#[post("/signup", data = "<signup>")]
pub async fn signup(
    signup: Json<Login>,
    cookies: &CookieJar<'_>,
    pool: &State<PgPool>,
) -> Result<(), ()> {
    let signup = signup.0;
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let phc = argon2
        .hash_password(signup.password.as_bytes(), &salt)
        .map_err(|e| println!("{e}"))?
        .to_string();

    let id: i32 = sqlx::query!(
        "INSERT INTO clients (username, phc) VALUES ($1, $2) RETURNING id",
        signup.username,
        phc.to_string()
    )
    .fetch_one(
        pool.acquire()
            .await
            .map_err(|e| {
                println!("{e}");
            })?
            .acquire()
            .await
            .map_err(|e| println!("{e}"))?,
    )
    .await
    .map_err(|e| eprintln!("{e}"))?
    .id;

    let token = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO tokens (token, client) VALUES ($1, $2)",
        &token.to_string(),
        id
    )
    .execute(
        pool.acquire()
            .await
            .map_err(|e| {
                println!("{e}");
            })?
            .acquire()
            .await
            .map_err(|e| println!("{e}"))?,
    )
    .await
    .map_err(|e| println!("{e}"))?;
    cookies.add(("token", token.to_string()));
    Ok(())
}
