use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateBreakTypeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub break_type: BreakType,
    pub idempotency_key: Option<String>,
}
impl<'a> CreateBreakTypeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateBreakTypeResponse> {
        let mut r = self.http_client.client.post("/v2/labor/break-types");
        r = r.json(json!({ "break_type" : self.break_type }));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateBreakTypeRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateBreakTypeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}