use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelPaymentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub payment_id: String,
}
impl<'a> CancelPaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CancelPaymentResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/payments/{payment_id}/cancel", payment_id = self.payment_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CancelPaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelPaymentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}