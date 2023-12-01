#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let dispute_id = "your dispute id";
    let evidence_text = "your evidence text";
    let idempotency_key = "your idempotency key";
    let response = client
        .create_dispute_evidence_text(dispute_id, evidence_text, idempotency_key)
        .evidence_type("your evidence type")
        .await
        .unwrap();
    println!("{:#?}", response);
}