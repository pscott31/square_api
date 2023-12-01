#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .update_subscription(subscription_id)
        .subscription(Subscription {
            actions: Some(
                vec![
                    SubscriptionAction { effective_date : Some("your effective date"
                    .to_owned()), id : Some("your id".to_owned()), new_plan_id :
                    Some("your new plan id".to_owned()), type_ : Some("your type"
                    .to_owned()) }
                ],
            ),
            canceled_date: Some("your canceled date".to_owned()),
            card_id: Some("your card id".to_owned()),
            charged_through_date: Some("your charged through date".to_owned()),
            created_at: Some("your created at".to_owned()),
            customer_id: Some("your customer id".to_owned()),
            id: Some("your id".to_owned()),
            invoice_ids: Some(vec!["your invoice ids".to_owned()]),
            location_id: Some("your location id".to_owned()),
            paid_until_date: Some("your paid until date".to_owned()),
            plan_id: Some("your plan id".to_owned()),
            price_override_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            source: Some(SubscriptionSource {
                name: Some("your name".to_owned()),
            }),
            start_date: Some("your start date".to_owned()),
            status: Some("your status".to_owned()),
            tax_percentage: Some("your tax percentage".to_owned()),
            timezone: Some("your timezone".to_owned()),
            version: Some(1),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}