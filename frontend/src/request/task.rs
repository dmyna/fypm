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
