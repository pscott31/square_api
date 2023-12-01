#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let domain_name = "your domain name";
    let response = client.register_domain(domain_name).await.unwrap();
    println!("{:#?}", response);
}