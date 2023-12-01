#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let group = CustomerGroup {
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        name: "your name".to_owned(),
        updated_at: Some("your updated at".to_owned()),
    };
    let group_id = "your group id";
    let response = client.update_customer_group(group, group_id).await.unwrap();
    println!("{:#?}", response);
}