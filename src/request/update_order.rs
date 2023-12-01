use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateOrderRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub fields_to_clear: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
    pub order: Option<Order>,
    pub order_id: String,
}
impl<'a> UpdateOrderRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateOrderResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/orders/{order_id}", order_id = self.order_id));
        if let Some(ref unwrapped) = self.fields_to_clear {
            r = r.json(json!({ "fields_to_clear" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order {
            r = r.json(json!({ "order" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn fields_to_clear(
        mut self,
        fields_to_clear: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .fields_to_clear = Some(
            fields_to_clear.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
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
impl<'a> ::std::future::IntoFuture for UpdateOrderRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateOrderResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}