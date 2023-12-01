use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteLoyaltyRewardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub reward_id: String,
}
impl<'a> DeleteLoyaltyRewardRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeleteLoyaltyRewardResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!("/v2/loyalty/rewards/{reward_id}", reward_id = self.reward_id),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteLoyaltyRewardRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteLoyaltyRewardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}