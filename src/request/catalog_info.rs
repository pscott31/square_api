use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CatalogInfoRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
}
impl<'a> CatalogInfoRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CatalogInfoResponse> {
        let mut r = self.http_client.client.get("/v2/catalog/info");
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CatalogInfoRequest<'a> {
    type Output = httpclient::InMemoryResult<CatalogInfoResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}