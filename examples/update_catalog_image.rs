#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let image_id = "your image id";
    let response = client.update_catalog_image(image_id).await.unwrap();
    println!("{:#?}", response);
}