#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let loyalty_promotion = LoyaltyPromotion {
        available_time: LoyaltyPromotionAvailableTimeData {
            end_date: Some("your end date".to_owned()),
            start_date: Some("your start date".to_owned()),
            time_periods: vec!["your time periods".to_owned()],
        },
        canceled_at: Some("your canceled at".to_owned()),
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        incentive: LoyaltyPromotionIncentive {
            points_addition_data: Some(LoyaltyPromotionIncentivePointsAdditionData {
                points_addition: 1,
            }),
            points_multiplier_data: Some(LoyaltyPromotionIncentivePointsMultiplierData {
                points_multiplier: 1,
            }),
            type_: "your type".to_owned(),
        },
        loyalty_program_id: Some("your loyalty program id".to_owned()),
        minimum_spend_amount_money: Some(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        }),
        name: "your name".to_owned(),
        status: Some("your status".to_owned()),
        trigger_limit: Some(LoyaltyPromotionTriggerLimit {
            interval: Some("your interval".to_owned()),
            times: 1,
        }),
        updated_at: Some("your updated at".to_owned()),
    };
    let program_id = "your program id";
    let response = client
        .create_loyalty_promotion(idempotency_key, loyalty_promotion, program_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}