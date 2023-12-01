use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PayOrderRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub order_id: String,
    pub order_version: Option<i64>,
    pub payment_ids: Option<Vec<String>>,
}
impl<'a> PayOrderRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<PayOrderResponse> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v2/orders/{order_id}/pay", order_id = self.order_id));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.order_version {
            r = r.json(json!({ "order_version" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_ids {
            r = r.json(json!({ "payment_ids" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn order_version(mut self, order_version: i64) -> Self {
        self.order_version = Some(order_version);
        self
    }
    pub fn payment_ids(
        mut self,
        payment_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .payment_ids = Some(
            payment_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for PayOrderRequest<'a> {
    type Output = httpclient::InMemoryResult<PayOrderResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}