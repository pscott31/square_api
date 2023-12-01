#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let site_id = "your site id";
    let snippet = Snippet {
        content: "your content".to_owned(),
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        site_id: Some("your site id".to_owned()),
        updated_at: Some("your updated at".to_owned()),
    };
    let response = client.upsert_snippet(site_id, snippet).await.unwrap();
    println!("{:#?}", response);
}