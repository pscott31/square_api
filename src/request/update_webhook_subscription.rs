use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub subscription: Option<WebhookSubscription>,
    pub subscription_id: String,
}
impl<'a> UpdateWebhookSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpdateWebhookSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/v2/webhooks/subscriptions/{subscription_id}", subscription_id =
                    self.subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.subscription {
            r = r.json(json!({ "subscription" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn subscription(mut self, subscription: WebhookSubscription) -> Self {
        self.subscription = Some(subscription);
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateWebhookSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}