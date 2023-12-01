use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListGiftCardsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub customer_id: Option<String>,
    pub limit: Option<i64>,
    pub state: Option<String>,
    pub type_: Option<String>,
}
impl<'a> ListGiftCardsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListGiftCardsResponse> {
        let mut r = self.http_client.client.get("/v2/gift-cards");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_id {
            r = r.query("customer_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.state {
            r = r.query("state", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn customer_id(mut self, customer_id: &str) -> Self {
        self.customer_id = Some(customer_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListGiftCardsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListGiftCardsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}