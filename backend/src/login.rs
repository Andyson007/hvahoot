use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use rocket::{
    State,
    futures::TryFutureExt,
    http::{CookieJar, Status},
    post,
    request::{FromRequest, Outcome, Request},
    serde::{Deserialize, json::Json},
};
use sqlx::{Acquire, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    username: String,
    password: String,
}

pub struct User {
    pub username: String,
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(pool) = req.guard::<&State<PgPool>>().await.succeeded() else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Some(token) = req.cookies().get("token") else {
            return Outcome::Error((Status::Forbidden, ()));
        };
        let Ok(mut pool_connection) = pool.acquire().await.map_err(|e| {
            println!("{e}");
        }) else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Ok(connection) = pool_connection.acquire().await.map_err(|e| println!("{e}")) else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        let Ok(record) = sqlx::query!(
            r#"SELECT username, id
        FROM tokens
            INNER JOIN clients
                ON clients.id = tokens.client
        WHERE token=$1 AND expires>STATEMENT_TIMESTAMP()"#,
            token.value()
        )
        .fetch_optional(connection)
        .await
        else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        if let Some(record) = record {
            Outcome::Success(User {
                username: record.username,
                id: record.id,
            })
        } else {
            return Outcome::Error((Status::Unauthorized, ()));
        }
    }
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
