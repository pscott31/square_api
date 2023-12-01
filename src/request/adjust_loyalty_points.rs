use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AdjustLoyaltyPointsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub account_id: String,
    pub adjust_points: LoyaltyEventAdjustPoints,
    pub allow_negative_balance: Option<bool>,
    pub idempotency_key: String,
}
impl<'a> AdjustLoyaltyPointsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<AdjustLoyaltyPointsResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/loyalty/accounts/{account_id}/adjust", account_id = self
                    .account_id
                ),
            );
        r = r.json(json!({ "adjust_points" : self.adjust_points }));
        if let Some(ref unwrapped) = self.allow_negative_balance {
            r = r.json(json!({ "allow_negative_balance" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn allow_negative_balance(mut self, allow_negative_balance: bool) -> Self {
        self.allow_negative_balance = Some(allow_negative_balance);
        self
    }
}
impl<'a> ::std::future::IntoFuture for AdjustLoyaltyPointsRequest<'a> {
    type Output = httpclient::InMemoryResult<AdjustLoyaltyPointsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}