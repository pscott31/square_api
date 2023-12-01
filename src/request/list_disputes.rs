use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListDisputesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub location_id: Option<String>,
    pub states: Option<String>,
}
impl<'a> ListDisputesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListDisputesResponse> {
        let mut r = self.http_client.client.get("/v2/disputes");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_id {
            r = r.query("location_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.states {
            r = r.query("states", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_owned());
        self
    }
    pub fn states(mut self, states: &str) -> Self {
        self.states = Some(states.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListDisputesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListDisputesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}