use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub subscription_id: String,
}
impl<'a> RetrieveWebhookSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveWebhookSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/webhooks/subscriptions/{subscription_id}", subscription_id =
                    self.subscription_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveWebhookSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}