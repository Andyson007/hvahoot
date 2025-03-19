extern crate rocket;

use std::{
    fs,
    path::{Path, PathBuf},
};

use backend::login::{login, signup};
use rocket::{fs::NamedFile, get, routes};
use sqlx::PgPool;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let Ok(pool) = PgPool::connect("postgres://hvahoot:aoeu@localHost:5432/hvahoot").await else {
        println!("failed to connect to pool\nexiting");
        return Ok(());
    };

    rocket::build()
        .manage(pool)
        .mount("/", routes![index, login, signup])
        .launch()
        .await?;
    Ok(())
}

#[get("/<path..>")]
async fn index(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("../frontend/build/").join(path);
    if fs::metadata(&path).ok()?.is_dir() {
        NamedFile::open(path.join("index.html")).await.ok()
    } else {
        NamedFile::open(path).await.ok()
    }
}
