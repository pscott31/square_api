use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchCatalogObjectsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub begin_time: Option<String>,
    pub cursor: Option<String>,
    pub include_deleted_objects: Option<bool>,
    pub include_related_objects: Option<bool>,
    pub limit: Option<i64>,
    pub object_types: Option<Vec<String>>,
    pub query: Option<CatalogQuery>,
}
impl<'a> SearchCatalogObjectsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SearchCatalogObjectsResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/search");
        if let Some(ref unwrapped) = self.begin_time {
            r = r.json(json!({ "begin_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_deleted_objects {
            r = r.json(json!({ "include_deleted_objects" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_related_objects {
            r = r.json(json!({ "include_related_objects" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.object_types {
            r = r.json(json!({ "object_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query {
            r = r.json(json!({ "query" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn begin_time(mut self, begin_time: &str) -> Self {
        self.begin_time = Some(begin_time.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn object_types(
        mut self,
        object_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .object_types = Some(
            object_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn query(mut self, query: CatalogQuery) -> Self {
        self.query = Some(query);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchCatalogObjectsRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchCatalogObjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}