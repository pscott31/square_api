use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BatchRetrieveCatalogObjectsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub catalog_version: Option<i64>,
    pub include_deleted_objects: Option<bool>,
    pub include_related_objects: Option<bool>,
    pub object_ids: Vec<String>,
}
impl<'a> BatchRetrieveCatalogObjectsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchRetrieveCatalogObjectsResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/batch-retrieve");
        if let Some(ref unwrapped) = self.catalog_version {
            r = r.json(json!({ "catalog_version" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_deleted_objects {
            r = r.json(json!({ "include_deleted_objects" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_related_objects {
            r = r.json(json!({ "include_related_objects" : unwrapped }));
        }
        r = r.json(json!({ "object_ids" : self.object_ids }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn catalog_version(mut self, catalog_version: i64) -> Self {
        self.catalog_version = Some(catalog_version);
        self
    }
    pub fn include_deleted_objects(mut self, include_deleted_objects: bool) -> Self {
        self.include_deleted_objects = Some(include_deleted_objects);
        self
    }
    pub fn include_related_objects(mut self, include_related_objects: bool) -> Self {
        self.include_related_objects = Some(include_related_objects);
        self
    }
}
impl<'a> ::std::future::IntoFuture for BatchRetrieveCatalogObjectsRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchRetrieveCatalogObjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}