use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub card: Card,
    pub idempotency_key: String,
    pub source_id: String,
    pub verification_token: Option<String>,
}
impl<'a> CreateCardRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateCardResponse> {
        let mut r = self.http_client.client.post("/v2/cards");
        r = r.json(json!({ "card" : self.card }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "source_id" : self.source_id }));
        if let Some(ref unwrapped) = self.verification_token {
            r = r.json(json!({ "verification_token" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn verification_token(mut self, verification_token: &str) -> Self {
        self.verification_token = Some(verification_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCardRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}