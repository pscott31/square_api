use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateShiftRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub id: String,
    pub shift: Shift,
}
impl<'a> UpdateShiftRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateShiftResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/labor/shifts/{id}", id = self.id));
        r = r.json(json!({ "shift" : self.shift }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateShiftRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateShiftResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}