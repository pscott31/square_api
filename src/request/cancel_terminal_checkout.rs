use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelTerminalCheckoutRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub checkout_id: String,
}
impl<'a> CancelTerminalCheckoutRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CancelTerminalCheckoutResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/terminals/checkouts/{checkout_id}/cancel", checkout_id = self
                    .checkout_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CancelTerminalCheckoutRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelTerminalCheckoutResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}