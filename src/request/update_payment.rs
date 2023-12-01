use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdatePaymentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub payment: Option<Payment>,
    pub payment_id: String,
}
impl<'a> UpdatePaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdatePaymentResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/payments/{payment_id}", payment_id = self.payment_id));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.payment {
            r = r.json(json!({ "payment" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn payment(mut self, payment: Payment) -> Self {
        self.payment = Some(payment);
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdatePaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdatePaymentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}