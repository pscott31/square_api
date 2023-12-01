use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BatchChangeInventoryRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub changes: Option<Vec<InventoryChange>>,
    pub idempotency_key: String,
    pub ignore_unchanged_counts: Option<bool>,
}
impl<'a> BatchChangeInventoryRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchChangeInventoryResponse> {
        let mut r = self.http_client.client.post("/v2/inventory/changes/batch-create");
        if let Some(ref unwrapped) = self.changes {
            r = r.json(json!({ "changes" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.ignore_unchanged_counts {
            r = r.json(json!({ "ignore_unchanged_counts" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn changes(mut self, changes: Vec<InventoryChange>) -> Self {
        self.changes = Some(changes);
        self
    }
    pub fn ignore_unchanged_counts(mut self, ignore_unchanged_counts: bool) -> Self {
        self.ignore_unchanged_counts = Some(ignore_unchanged_counts);
        self
    }
}
impl<'a> ::std::future::IntoFuture for BatchChangeInventoryRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchChangeInventoryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}