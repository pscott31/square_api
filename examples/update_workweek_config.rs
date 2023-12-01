#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let id = "your id";
    let workweek_config = WorkweekConfig {
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        start_of_day_local_time: "your start of day local time".to_owned(),
        start_of_week: "your start of week".to_owned(),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
    };
    let response = client.update_workweek_config(id, workweek_config).await.unwrap();
    println!("{:#?}", response);
}