use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListSitesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
}
impl<'a> ListSitesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListSitesResponse> {
        let mut r = self.http_client.client.get("/v2/sites");
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ListSitesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListSitesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}