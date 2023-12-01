use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetTerminalRefundRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub terminal_refund_id: String,
}
impl<'a> GetTerminalRefundRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<GetTerminalRefundResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/terminals/refunds/{terminal_refund_id}", terminal_refund_id =
                    self.terminal_refund_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetTerminalRefundRequest<'a> {
    type Output = httpclient::InMemoryResult<GetTerminalRefundResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}