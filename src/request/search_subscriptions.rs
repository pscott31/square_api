use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchSubscriptionsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub include: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub query: Option<SearchSubscriptionsQuery>,
}
impl<'a> SearchSubscriptionsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SearchSubscriptionsResponse> {
        let mut r = self.http_client.client.post("/v2/subscriptions/search");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include {
            r = r.json(json!({ "include" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query {
            r = r.json(json!({ "query" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn include(
        mut self,
        include: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .include = Some(
            include.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn query(mut self, query: SearchSubscriptionsQuery) -> Self {
        self.query = Some(query);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchSubscriptionsRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchSubscriptionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}