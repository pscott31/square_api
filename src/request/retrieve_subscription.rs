use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub include: Option<String>,
    pub subscription_id: String,
}
impl<'a> RetrieveSubscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/subscriptions/{subscription_id}", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.include {
            r = r.query("include", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn include(mut self, include: &str) -> Self {
        self.include = Some(include.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}