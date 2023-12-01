use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchOrdersRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub location_ids: Option<Vec<String>>,
    pub query: Option<SearchOrdersQuery>,
    pub return_entries: Option<bool>,
}
impl<'a> SearchOrdersRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SearchOrdersResponse> {
        let mut r = self.http_client.client.post("/v2/orders/search");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.location_ids {
            r = r.json(json!({ "location_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query {
            r = r.json(json!({ "query" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_entries {
            r = r.json(json!({ "return_entries" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn location_ids(
        mut self,
        location_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .location_ids = Some(
            location_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn query(mut self, query: SearchOrdersQuery) -> Self {
        self.query = Some(query);
        self
    }
    pub fn return_entries(mut self, return_entries: bool) -> Self {
        self.return_entries = Some(return_entries);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchOrdersRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchOrdersResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}