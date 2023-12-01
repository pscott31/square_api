use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelLoyaltyPromotionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub program_id: String,
    pub promotion_id: String,
}
impl<'a> CancelLoyaltyPromotionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CancelLoyaltyPromotionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/loyalty/programs/{program_id}/promotions/{promotion_id}/cancel",
                    program_id = self.program_id, promotion_id = self.promotion_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CancelLoyaltyPromotionRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelLoyaltyPromotionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}