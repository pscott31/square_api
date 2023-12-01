use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchAvailabilityRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub query: SearchAvailabilityQuery,
}
impl<'a> SearchAvailabilityRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SearchAvailabilityResponse> {
        let mut r = self.http_client.client.post("/v2/bookings/availability/search");
        r = r.json(json!({ "query" : self.query }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for SearchAvailabilityRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchAvailabilityResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}