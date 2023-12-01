use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CalculateLoyaltyPointsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub loyalty_account_id: Option<String>,
    pub order_id: Option<String>,
    pub program_id: String,
    pub transaction_amount_money: Option<Money>,
}
impl<'a> CalculateLoyaltyPointsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CalculateLoyaltyPointsResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/loyalty/programs/{program_id}/calculate", program_id = self
                    .program_id
                ),
            );
        if let Some(ref unwrapped) = self.loyalty_account_id {
            r = r.json(json!({ "loyalty_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order_id {
            r = r.json(json!({ "order_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transaction_amount_money {
            r = r.json(json!({ "transaction_amount_money" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn loyalty_account_id(mut self, loyalty_account_id: &str) -> Self {
        self.loyalty_account_id = Some(loyalty_account_id.to_owned());
        self
    }
    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_owned());
        self
    }
    pub fn transaction_amount_money(mut self, transaction_amount_money: Money) -> Self {
        self.transaction_amount_money = Some(transaction_amount_money);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CalculateLoyaltyPointsRequest<'a> {
    type Output = httpclient::InMemoryResult<CalculateLoyaltyPointsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}