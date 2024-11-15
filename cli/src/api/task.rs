use crate::utils::get;

#[derive(FromForm)]
pub struct TaskQuery {
    pub filter: Option<String>,
}

#[get("/task?<params..>")]
pub fn get_task(params: TaskQuery) -> String {
    let filter = if let Some(filter) = params.filter {
        filter
    } else {
        "".to_string()
    };

    let data = get::json_by_filter(&filter, None).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
