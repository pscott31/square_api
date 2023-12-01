use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelTerminalActionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub action_id: String,
}
impl<'a> CancelTerminalActionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CancelTerminalActionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/terminals/actions/{action_id}/cancel", action_id = self
                    .action_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CancelTerminalActionRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelTerminalActionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}