use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub subscription_id: String,
}
impl<'a> DeleteWebhookSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeleteWebhookSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/webhooks/subscriptions/{subscription_id}", subscription_id =
                    self.subscription_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteWebhookSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}