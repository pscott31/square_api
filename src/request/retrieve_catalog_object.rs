use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveCatalogObjectRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub catalog_version: Option<i64>,
    pub include_related_objects: Option<bool>,
    pub object_id: String,
}
impl<'a> RetrieveCatalogObjectRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveCatalogObjectResponse> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v2/catalog/object/{object_id}", object_id = self.object_id));
        if let Some(ref unwrapped) = self.catalog_version {
            r = r.query("catalog_version", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.include_related_objects {
            r = r.query("include_related_objects", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn catalog_version(mut self, catalog_version: i64) -> Self {
        self.catalog_version = Some(catalog_version);
        self
    }
    pub fn include_related_objects(mut self, include_related_objects: bool) -> Self {
        self.include_related_objects = Some(include_related_objects);
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveCatalogObjectRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveCatalogObjectResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}