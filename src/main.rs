
use rocket::http::hyper::uri;
use rocket::{*, response::content::RawCss};

use rocket::response::content::RawHtml;
use minijinja::{Environment, context};

pub mod routes;

#[rocket::main]
async fn main() -> Result<(), Error>{
    let rocket = rocket::build()
        .mount("/", routes![routes::home::index])
        .launch()
        .await?;
    Ok(())
}