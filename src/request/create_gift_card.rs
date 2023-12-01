use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateGiftCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub gift_card: GiftCard,
    pub idempotency_key: String,
    pub location_id: String,
}
impl<'a> CreateGiftCardRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateGiftCardResponse> {
        let mut r = self.http_client.client.post("/v2/gift-cards");
        r = r.json(json!({ "gift_card" : self.gift_card }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "location_id" : self.location_id }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateGiftCardRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateGiftCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}