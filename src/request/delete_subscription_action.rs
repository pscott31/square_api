use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteSubscriptionActionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub action_id: String,
    pub subscription_id: String,
}
impl<'a> DeleteSubscriptionActionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeleteSubscriptionActionResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/subscriptions/{subscription_id}/actions/{action_id}", action_id
                    = self.action_id, subscription_id = self.subscription_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteSubscriptionActionRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteSubscriptionActionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}