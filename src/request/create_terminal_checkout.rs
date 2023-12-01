use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTerminalCheckoutRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub checkout: TerminalCheckout,
    pub idempotency_key: String,
}
impl<'a> CreateTerminalCheckoutRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateTerminalCheckoutResponse> {
        let mut r = self.http_client.client.post("/v2/terminals/checkouts");
        r = r.json(json!({ "checkout" : self.checkout }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateTerminalCheckoutRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTerminalCheckoutResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}