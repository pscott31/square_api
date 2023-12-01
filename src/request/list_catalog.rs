use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCatalogRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub catalog_version: Option<i64>,
    pub cursor: Option<String>,
    pub types: Option<String>,
}
impl<'a> ListCatalogRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListCatalogResponse> {
        let mut r = self.http_client.client.get("/v2/catalog/list");
        if let Some(ref unwrapped) = self.catalog_version {
            r = r.query("catalog_version", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.types {
            r = r.query("types", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn catalog_version(mut self, catalog_version: i64) -> Self {
        self.catalog_version = Some(catalog_version);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn types(mut self, types: &str) -> Self {
        self.types = Some(types.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCatalogRequest<'a> {
    type Output = httpclient::InMemoryResult<ListCatalogResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}