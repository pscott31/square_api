#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_customers()
        .cursor("your cursor")
        .limit(1)
        .query(CustomerQuery {
            filter: Some(CustomerFilter {
                created_at: Some(TimeRange {
                    end_at: Some("your end at".to_owned()),
                    start_at: Some("your start at".to_owned()),
                }),
                creation_source: Some(CustomerCreationSourceFilter {
                    rule: Some("your rule".to_owned()),
                    values: Some(vec!["your values".to_owned()]),
                }),
                custom_attribute: Some(CustomerCustomAttributeFilters {
                    filters: Some(
                        vec![
                            CustomerCustomAttributeFilter { filter : None, key :
                            "your key".to_owned(), updated_at : Some(TimeRange { end_at :
                            Some("your end at".to_owned()), start_at :
                            Some("your start at".to_owned()) }) }
                        ],
                    ),
                }),
                email_address: Some(CustomerTextFilter {
                    exact: Some("your exact".to_owned()),
                    fuzzy: Some("your fuzzy".to_owned()),
                }),
                group_ids: Some(FilterValue {
                    all: Some(vec!["your all".to_owned()]),
                    any: Some(vec!["your any".to_owned()]),
                    none: Some(vec!["your none".to_owned()]),
                }),
                phone_number: Some(CustomerTextFilter {
                    exact: Some("your exact".to_owned()),
                    fuzzy: Some("your fuzzy".to_owned()),
                }),
                reference_id: Some(CustomerTextFilter {
                    exact: Some("your exact".to_owned()),
                    fuzzy: Some("your fuzzy".to_owned()),
                }),
                updated_at: Some(TimeRange {
                    end_at: Some("your end at".to_owned()),
                    start_at: Some("your start at".to_owned()),
                }),
            }),
            sort: Some(CustomerSort {
                field: Some("your field".to_owned()),
                order: Some("your order".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}