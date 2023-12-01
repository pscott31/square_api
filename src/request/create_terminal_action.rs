use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTerminalActionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub action: TerminalAction,
    pub idempotency_key: String,
}
impl<'a> CreateTerminalActionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateTerminalActionResponse> {
        let mut r = self.http_client.client.post("/v2/terminals/actions");
        r = r.json(json!({ "action" : self.action }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateTerminalActionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTerminalActionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}