extern crate rocket;

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use backend::{
    hvahoot::{create_hvahoot, get_uuid, quiz, quizzes},
    login::{login, signup},
    play::{host, play, Game},
};
use rocket::{fs::NamedFile, get, routes, tokio::sync::RwLock};
use sqlx::PgPool;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let Ok(pool) = PgPool::connect(
        &env::var("DATABASE_URL").expect("please set the DATABASE_URL environment variable"),
    )
    .await
    else {
        println!("failed to connect to pool\nexiting");
        return Ok(());
    };

    rocket::build()
        .manage(pool)
        .manage(Arc::new(RwLock::new(HashMap::<u32, Game>::new())))
        .mount(
            "/",
            routes![
                index,
                login,
                signup,
                get_uuid,
                create_hvahoot,
                quiz,
                play,
                host,
                quizzes
            ],
        )
        .launch()
        .await?;
    Ok(())
}

#[get("/<path..>")]
async fn index(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("../frontend/build/").join(path);
    match fs::metadata(&path) {
        Ok(x) if x.is_dir() => NamedFile::open(path.join("index.html")).await.ok(),
        Ok(x) if x.is_file() => NamedFile::open(path).await.ok(),
        Err(_) => NamedFile::open(format!("{}.html", path.to_str()?))
            .await
            .ok(),
        _ => None,
    }
}
