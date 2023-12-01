use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveGiftCardFromNonceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub nonce: String,
}
impl<'a> RetrieveGiftCardFromNonceRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveGiftCardFromNonceResponse> {
        let mut r = self.http_client.client.post("/v2/gift-cards/from-nonce");
        r = r.json(json!({ "nonce" : self.nonce }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveGiftCardFromNonceRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveGiftCardFromNonceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}