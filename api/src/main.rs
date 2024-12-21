////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

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
