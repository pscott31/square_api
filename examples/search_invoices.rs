#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let query = InvoiceQuery {
        filter: InvoiceFilter {
            customer_ids: Some(vec!["your customer ids".to_owned()]),
            location_ids: vec!["your location ids".to_owned()],
        },
        sort: Some(InvoiceSort {
            field: "your field".to_owned(),
            order: Some("your order".to_owned()),
        }),
    };
    let response = client
        .search_invoices(query)
        .cursor("your cursor")
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}