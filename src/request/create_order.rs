use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateOrderRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: Option<String>,
    pub order: Option<Order>,
}
impl<'a> CreateOrderRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateOrderResponse> {
        let mut r = self.http_client.client.post("/v2/orders");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order {
            r = r.json(json!({ "order" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateOrderRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateOrderResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}