#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let response = client
        .deprecated_batch_change_inventory(idempotency_key)
        .changes(
            vec![
                InventoryChange { adjustment : Some(InventoryAdjustment {
                adjustment_group : Some(InventoryAdjustmentGroup { from_state :
                Some("your from state".to_owned()), id : Some("your id".to_owned()),
                root_adjustment_id : Some("your root adjustment id".to_owned()), to_state
                : Some("your to state".to_owned()) }), catalog_object_id :
                Some("your catalog object id".to_owned()), catalog_object_type :
                Some("your catalog object type".to_owned()), created_at :
                Some("your created at".to_owned()), employee_id : Some("your employee id"
                .to_owned()), from_location_id : Some("your from location id"
                .to_owned()), from_state : Some("your from state".to_owned()),
                from_status : Some("your from status".to_owned()), goods_receipt_id :
                Some("your goods receipt id".to_owned()), id : Some("your id"
                .to_owned()), location_id : Some("your location id".to_owned()),
                occurred_at : Some("your occurred at".to_owned()), purchase_order_id :
                Some("your purchase order id".to_owned()), quantity :
                Some("your quantity".to_owned()), reference_id : Some("your reference id"
                .to_owned()), refund_id : Some("your refund id".to_owned()), source :
                Some(SourceApplication { application_id : Some("your application id"
                .to_owned()), name : Some("your name".to_owned()), product :
                Some("your product".to_owned()) }), team_member_id :
                Some("your team member id".to_owned()), to_location_id :
                Some("your to location id".to_owned()), to_state : Some("your to state"
                .to_owned()), to_status : Some("your to status".to_owned()),
                total_price_money : Some(Money { amount : Some(1), currency :
                Some("your currency".to_owned()) }), transaction_id :
                Some("your transaction id".to_owned()) }), measurement_unit :
                Some(CatalogMeasurementUnit { measurement_unit : None, precision :
                Some(1) }), measurement_unit_id : Some("your measurement unit id"
                .to_owned()), physical_count : Some(InventoryPhysicalCount {
                catalog_object_id : Some("your catalog object id".to_owned()),
                catalog_object_type : Some("your catalog object type".to_owned()),
                created_at : Some("your created at".to_owned()), employee_id :
                Some("your employee id".to_owned()), id : Some("your id".to_owned()),
                location_id : Some("your location id".to_owned()), occurred_at :
                Some("your occurred at".to_owned()), quantity : Some("your quantity"
                .to_owned()), reference_id : Some("your reference id".to_owned()), source
                : Some(SourceApplication { application_id : Some("your application id"
                .to_owned()), name : Some("your name".to_owned()), product :
                Some("your product".to_owned()) }), state : Some("your state"
                .to_owned()), status : Some("your status".to_owned()), team_member_id :
                Some("your team member id".to_owned()) }), transfer :
                Some(InventoryTransfer { catalog_object_id :
                Some("your catalog object id".to_owned()), catalog_object_type :
                Some("your catalog object type".to_owned()), created_at :
                Some("your created at".to_owned()), employee_id : Some("your employee id"
                .to_owned()), from_location_id : Some("your from location id"
                .to_owned()), id : Some("your id".to_owned()), occurred_at :
                Some("your occurred at".to_owned()), quantity : Some("your quantity"
                .to_owned()), reference_id : Some("your reference id".to_owned()), source
                : Some(SourceApplication { application_id : Some("your application id"
                .to_owned()), name : Some("your name".to_owned()), product :
                Some("your product".to_owned()) }), state : Some("your state"
                .to_owned()), team_member_id : Some("your team member id".to_owned()),
                to_location_id : Some("your to location id".to_owned()) }), type_ :
                Some("your type".to_owned()) }
            ],
        )
        .ignore_unchanged_counts(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}