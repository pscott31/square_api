#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let gift_card_activity = GiftCardActivity {
        activate_activity_details: Some(GiftCardActivityActivate {
            amount_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            buyer_payment_instrument_ids: Some(
                vec!["your buyer payment instrument ids".to_owned()],
            ),
            line_item_uid: Some("your line item uid".to_owned()),
            order_id: Some("your order id".to_owned()),
            reference_id: Some("your reference id".to_owned()),
        }),
        adjust_decrement_activity_details: Some(GiftCardActivityAdjustDecrement {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            reason: "your reason".to_owned(),
        }),
        adjust_increment_activity_details: Some(GiftCardActivityAdjustIncrement {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            reason: "your reason".to_owned(),
        }),
        block_activity_details: Some(GiftCardActivityBlock {
            reason: "your reason".to_owned(),
        }),
        clear_balance_activity_details: Some(GiftCardActivityClearBalance {
            reason: "your reason".to_owned(),
        }),
        created_at: Some("your created at".to_owned()),
        deactivate_activity_details: Some(GiftCardActivityDeactivate {
            reason: "your reason".to_owned(),
        }),
        gift_card_balance_money: Some(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        }),
        gift_card_gan: Some("your gift card gan".to_owned()),
        gift_card_id: Some("your gift card id".to_owned()),
        id: Some("your id".to_owned()),
        import_activity_details: Some(GiftCardActivityImport {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
        }),
        import_reversal_activity_details: Some(GiftCardActivityImportReversal {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
        }),
        load_activity_details: Some(GiftCardActivityLoad {
            amount_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            buyer_payment_instrument_ids: Some(
                vec!["your buyer payment instrument ids".to_owned()],
            ),
            line_item_uid: Some("your line item uid".to_owned()),
            order_id: Some("your order id".to_owned()),
            reference_id: Some("your reference id".to_owned()),
        }),
        location_id: "your location id".to_owned(),
        redeem_activity_details: Some(GiftCardActivityRedeem {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            payment_id: Some("your payment id".to_owned()),
            reference_id: Some("your reference id".to_owned()),
            status: Some("your status".to_owned()),
        }),
        refund_activity_details: Some(GiftCardActivityRefund {
            amount_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            payment_id: Some("your payment id".to_owned()),
            redeem_activity_id: Some("your redeem activity id".to_owned()),
            reference_id: Some("your reference id".to_owned()),
        }),
        type_: "your type".to_owned(),
        unblock_activity_details: Some(GiftCardActivityUnblock {
            reason: "your reason".to_owned(),
        }),
        unlinked_activity_refund_activity_details: Some(GiftCardActivityUnlinkedActivityRefund {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            payment_id: Some("your payment id".to_owned()),
            reference_id: Some("your reference id".to_owned()),
        }),
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .create_gift_card_activity(gift_card_activity, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}