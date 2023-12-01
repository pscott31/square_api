use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TestWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub event_type: Option<String>,
    pub subscription_id: String,
}
impl<'a> TestWebhookSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TestWebhookSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/webhooks/subscriptions/{subscription_id}/test", subscription_id
                    = self.subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.event_type {
            r = r.json(json!({ "event_type" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TestWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<TestWebhookSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}