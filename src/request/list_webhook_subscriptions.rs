use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListWebhookSubscriptionsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub include_disabled: Option<bool>,
    pub limit: Option<i64>,
    pub sort_order: Option<String>,
}
impl<'a> ListWebhookSubscriptionsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListWebhookSubscriptionsResponse> {
        let mut r = self.http_client.client.get("/v2/webhooks/subscriptions");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.include_disabled {
            r = r.query("include_disabled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_order {
            r = r.query("sort_order", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn include_disabled(mut self, include_disabled: bool) -> Self {
        self.include_disabled = Some(include_disabled);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn sort_order(mut self, sort_order: &str) -> Self {
        self.sort_order = Some(sort_order.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListWebhookSubscriptionsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListWebhookSubscriptionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}