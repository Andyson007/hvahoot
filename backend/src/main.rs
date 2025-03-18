use rocket::{get, routes};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build().mount("/", routes![index]).launch().await?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "nk"
}
