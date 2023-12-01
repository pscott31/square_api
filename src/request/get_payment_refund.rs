use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetPaymentRefundRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub refund_id: String,
}
impl<'a> GetPaymentRefundRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<GetPaymentRefundResponse> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v2/refunds/{refund_id}", refund_id = self.refund_id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetPaymentRefundRequest<'a> {
    type Output = httpclient::InMemoryResult<GetPaymentRefundResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}