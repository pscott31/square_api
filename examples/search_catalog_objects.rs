#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_catalog_objects()
        .begin_time("your begin time")
        .cursor("your cursor")
        .include_deleted_objects(true)
        .include_related_objects(true)
        .limit(1)
        .object_types(&["your object types"])
        .query(CatalogQuery {
            exact_query: Some(CatalogQueryExact {
                attribute_name: "your attribute name".to_owned(),
                attribute_value: "your attribute value".to_owned(),
            }),
            item_variations_for_item_option_values_query: Some(CatalogQueryItemVariationsForItemOptionValues {
                item_option_value_ids: Some(
                    vec!["your item option value ids".to_owned()],
                ),
            }),
            items_for_item_options_query: Some(CatalogQueryItemsForItemOptions {
                item_option_ids: Some(vec!["your item option ids".to_owned()]),
            }),
            items_for_modifier_list_query: Some(CatalogQueryItemsForModifierList {
                modifier_list_ids: vec!["your modifier list ids".to_owned()],
            }),
            items_for_tax_query: Some(CatalogQueryItemsForTax {
                tax_ids: vec!["your tax ids".to_owned()],
            }),
            prefix_query: Some(CatalogQueryPrefix {
                attribute_name: "your attribute name".to_owned(),
                attribute_prefix: "your attribute prefix".to_owned(),
            }),
            range_query: Some(CatalogQueryRange {
                attribute_max_value: Some(1),
                attribute_min_value: Some(1),
                attribute_name: "your attribute name".to_owned(),
            }),
            set_query: Some(CatalogQuerySet {
                attribute_name: "your attribute name".to_owned(),
                attribute_values: vec!["your attribute values".to_owned()],
            }),
            sorted_attribute_query: Some(CatalogQuerySortedAttribute {
                attribute_name: "your attribute name".to_owned(),
                initial_attribute_value: Some("your initial attribute value".to_owned()),
                sort_order: Some("your sort order".to_owned()),
            }),
            text_query: Some(CatalogQueryText {
                keywords: vec!["your keywords".to_owned()],
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}