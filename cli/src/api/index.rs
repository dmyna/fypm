use std::str::FromStr;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};


#[rocket::main]
pub async fn rocket() -> Result<(), rocket::Error> {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: ["GET"].iter().map(|s| FromStr::from_str(s).unwrap()).collect(),
        allowed_headers: AllowedHeaders::All,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create cors");

    rocket::build()
        .attach(cors)
        .mount("/api", routes![super::time::listing])
        .launch()
        .await?;

    Ok(())
}
