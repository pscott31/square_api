use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeprecatedBatchRetrieveInventoryChangesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub catalog_object_ids: Option<Vec<String>>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub location_ids: Option<Vec<String>>,
    pub states: Option<Vec<String>>,
    pub statuses: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
}
impl<'a> DeprecatedBatchRetrieveInventoryChangesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchRetrieveInventoryChangesResponse> {
        let mut r = self.http_client.client.post("/v2/inventory/batch-retrieve-changes");
        if let Some(ref unwrapped) = self.catalog_object_ids {
            r = r.json(json!({ "catalog_object_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.location_ids {
            r = r.json(json!({ "location_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.states {
            r = r.json(json!({ "states" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.statuses {
            r = r.json(json!({ "statuses" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.types {
            r = r.json(json!({ "types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.updated_after {
            r = r.json(json!({ "updated_after" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.updated_before {
            r = r.json(json!({ "updated_before" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn catalog_object_ids(
        mut self,
        catalog_object_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .catalog_object_ids = Some(
            catalog_object_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn location_ids(
        mut self,
        location_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .location_ids = Some(
            location_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn states(mut self, states: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.states = Some(states.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn statuses(
        mut self,
        statuses: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .statuses = Some(
            statuses.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn types(mut self, types: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.types = Some(types.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn updated_after(mut self, updated_after: &str) -> Self {
        self.updated_after = Some(updated_after.to_owned());
        self
    }
    pub fn updated_before(mut self, updated_before: &str) -> Self {
        self.updated_before = Some(updated_before.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for DeprecatedBatchRetrieveInventoryChangesRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchRetrieveInventoryChangesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}