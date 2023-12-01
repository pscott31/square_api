use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchVendorsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub filter: Option<SearchVendorsRequestFilter>,
    pub sort: Option<SearchVendorsRequestSort>,
}
impl<'a> SearchVendorsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SearchVendorsResponse> {
        let mut r = self.http_client.client.post("/v2/vendors/search");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.filter {
            r = r.json(json!({ "filter" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.json(json!({ "sort" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn filter(mut self, filter: SearchVendorsRequestFilter) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn sort(mut self, sort: SearchVendorsRequestSort) -> Self {
        self.sort = Some(sort);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchVendorsRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchVendorsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}