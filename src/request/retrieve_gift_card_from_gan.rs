use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveGiftCardFromGanRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub gan: String,
}
impl<'a> RetrieveGiftCardFromGanRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveGiftCardFromGanResponse> {
        let mut r = self.http_client.client.post("/v2/gift-cards/from-gan");
        r = r.json(json!({ "gan" : self.gan }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveGiftCardFromGanRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveGiftCardFromGanResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}