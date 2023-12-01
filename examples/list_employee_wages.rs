#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_employee_wages()
        .cursor("your cursor")
        .employee_id("your employee id")
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}