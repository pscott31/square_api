use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateLoyaltyPromotionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub loyalty_promotion: LoyaltyPromotion,
    pub program_id: String,
}
impl<'a> CreateLoyaltyPromotionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateLoyaltyPromotionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/loyalty/programs/{program_id}/promotions", program_id = self
                    .program_id
                ),
            );
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "loyalty_promotion" : self.loyalty_promotion }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateLoyaltyPromotionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateLoyaltyPromotionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}