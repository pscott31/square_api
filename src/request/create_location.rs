use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateLocationRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub location: Option<Location>,
}
impl<'a> CreateLocationRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateLocationResponse> {
        let mut r = self.http_client.client.post("/v2/locations");
        if let Some(ref unwrapped) = self.location {
            r = r.json(json!({ "location" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateLocationRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateLocationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}