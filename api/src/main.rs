#[macro_use]
extern crate rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::str::FromStr;

mod routes;
mod utils;

//#region           Implementation
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: ["GET"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::All,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create cors");

    rocket::build()
        .attach(cors)
        .mount(
            "/api",
            routes![routes::time::listing, routes::task::get_task],
        )
        .launch()
        .await?;

    Ok(())
}
//#endregion
