use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTerminalRefundRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub refund: Option<TerminalRefund>,
}
impl<'a> CreateTerminalRefundRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateTerminalRefundResponse> {
        let mut r = self.http_client.client.post("/v2/terminals/refunds");
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.refund {
            r = r.json(json!({ "refund" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn refund(mut self, refund: TerminalRefund) -> Self {
        self.refund = Some(refund);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateTerminalRefundRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTerminalRefundResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}