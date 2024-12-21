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

use url::Url;

use fypm_lib::values::structs::TaskWarriorExported;

use crate::API_PORT;
use reqwest::Error;

pub async fn get_by_filter(filter: String) -> Result<Vec<TaskWarriorExported>, Error> {
    let api_url = format!("http://localhost:{}/api", API_PORT.to_string());
    let mut url = Url::parse(format!("{}/task", api_url).as_str()).unwrap();

    url.query_pairs_mut()
        .append_pair("filter", filter.as_str());

    let response = reqwest::get(url.as_str()).await;

    match response {
        Ok(response) => {
            let tasks = serde_json::from_str::<Vec<TaskWarriorExported>>(
                response.text().await.unwrap().as_str(),
            )
            .expect("Failed to parse time logs to json!");

            Ok(tasks)
        }
        Err(e) => Err(e),
    }
}
