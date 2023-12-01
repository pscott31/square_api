use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListWebhookEventTypesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub api_version: Option<String>,
}
impl<'a> ListWebhookEventTypesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListWebhookEventTypesResponse> {
        let mut r = self.http_client.client.get("/v2/webhooks/event-types");
        if let Some(ref unwrapped) = self.api_version {
            r = r.query("api_version", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn api_version(mut self, api_version: &str) -> Self {
        self.api_version = Some(api_version.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListWebhookEventTypesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListWebhookEventTypesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}