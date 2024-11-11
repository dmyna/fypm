#[rocket::main]
pub async fn rocket() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api", routes![super::time::listing])
        .launch()
        .await?;

    Ok(())
}
