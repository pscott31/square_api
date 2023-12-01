use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AccumulateLoyaltyPointsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub account_id: String,
    pub accumulate_points: LoyaltyEventAccumulatePoints,
    pub idempotency_key: String,
    pub location_id: String,
}
impl<'a> AccumulateLoyaltyPointsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<AccumulateLoyaltyPointsResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/loyalty/accounts/{account_id}/accumulate", account_id = self
                    .account_id
                ),
            );
        r = r.json(json!({ "accumulate_points" : self.accumulate_points }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "location_id" : self.location_id }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
pub struct AccumulateLoyaltyPointsRequired<'a> {
    pub account_id: &'a str,
    pub accumulate_points: LoyaltyEventAccumulatePoints,
    pub idempotency_key: &'a str,
    pub location_id: &'a str,
}
impl<'a> AccumulateLoyaltyPointsRequired<'a> {}
impl<'a> ::std::future::IntoFuture for AccumulateLoyaltyPointsRequest<'a> {
    type Output = httpclient::InMemoryResult<AccumulateLoyaltyPointsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}