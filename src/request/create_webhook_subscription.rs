use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: Option<String>,
    pub subscription: WebhookSubscription,
}
impl<'a> CreateWebhookSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateWebhookSubscriptionResponse> {
        let mut r = self.http_client.client.post("/v2/webhooks/subscriptions");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        r = r.json(json!({ "subscription" : self.subscription }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateWebhookSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}