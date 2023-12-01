use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DisableCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub card_id: String,
}
impl<'a> DisableCardRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DisableCardResponse> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v2/cards/{card_id}/disable", card_id = self.card_id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DisableCardRequest<'a> {
    type Output = httpclient::InMemoryResult<DisableCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}