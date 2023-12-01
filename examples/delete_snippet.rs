#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let site_id = "your site id";
    let response = client.delete_snippet(site_id).await.unwrap();
    println!("{:#?}", response);
}