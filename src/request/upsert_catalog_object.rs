use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpsertCatalogObjectRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub object: CatalogObject,
}
impl<'a> UpsertCatalogObjectRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpsertCatalogObjectResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/object");
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "object" : self.object }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpsertCatalogObjectRequest<'a> {
    type Output = httpclient::InMemoryResult<UpsertCatalogObjectResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}