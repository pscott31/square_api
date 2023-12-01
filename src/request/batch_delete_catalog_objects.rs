use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BatchDeleteCatalogObjectsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub object_ids: Option<Vec<String>>,
}
impl<'a> BatchDeleteCatalogObjectsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchDeleteCatalogObjectsResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/batch-delete");
        if let Some(ref unwrapped) = self.object_ids {
            r = r.json(json!({ "object_ids" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn object_ids(
        mut self,
        object_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .object_ids = Some(
            object_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for BatchDeleteCatalogObjectsRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchDeleteCatalogObjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}