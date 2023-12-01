use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateBreakTypeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub break_type: BreakType,
    pub id: String,
}
impl<'a> UpdateBreakTypeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateBreakTypeResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/labor/break-types/{id}", id = self.id));
        r = r.json(json!({ "break_type" : self.break_type }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateBreakTypeRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateBreakTypeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}