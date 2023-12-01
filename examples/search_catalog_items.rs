#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_catalog_items()
        .category_ids(&["your category ids"])
        .cursor("your cursor")
        .custom_attribute_filters(
            vec![
                CustomAttributeFilter { bool_filter : Some(true),
                custom_attribute_definition_id :
                Some("your custom attribute definition id".to_owned()), key :
                Some("your key".to_owned()), number_filter : Some(Range { max :
                Some("your max".to_owned()), min : Some("your min".to_owned()) }),
                selection_uids_filter : Some(vec!["your selection uids filter"
                .to_owned()]), string_filter : Some("your string filter".to_owned()) }
            ],
        )
        .enabled_location_ids(&["your enabled location ids"])
        .limit(1)
        .product_types(&["your product types"])
        .sort_order("your sort order")
        .stock_levels(&["your stock levels"])
        .text_filter("your text filter")
        .await
        .unwrap();
    println!("{:#?}", response);
}