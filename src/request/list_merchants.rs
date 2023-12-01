use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListMerchantsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<i64>,
}
impl<'a> ListMerchantsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListMerchantsResponse> {
        let mut r = self.http_client.client.get("/v2/merchants");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: i64) -> Self {
        self.cursor = Some(cursor);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListMerchantsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListMerchantsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}