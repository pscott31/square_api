use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BatchUpsertCatalogObjectsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub batches: Vec<CatalogObjectBatch>,
    pub idempotency_key: String,
}
impl<'a> BatchUpsertCatalogObjectsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchUpsertCatalogObjectsResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/batch-upsert");
        r = r.json(json!({ "batches" : self.batches }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for BatchUpsertCatalogObjectsRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchUpsertCatalogObjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}