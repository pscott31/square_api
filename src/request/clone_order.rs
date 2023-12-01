use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CloneOrderRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: Option<String>,
    pub order_id: String,
    pub version: Option<i64>,
}
impl<'a> CloneOrderRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CloneOrderResponse> {
        let mut r = self.http_client.client.post("/v2/orders/clone");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        r = r.json(json!({ "order_id" : self.order_id }));
        if let Some(ref unwrapped) = self.version {
            r = r.json(json!({ "version" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CloneOrderRequest<'a> {
    type Output = httpclient::InMemoryResult<CloneOrderResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}