use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCardsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub customer_id: Option<String>,
    pub include_disabled: Option<bool>,
    pub reference_id: Option<String>,
    pub sort_order: Option<String>,
}
impl<'a> ListCardsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListCardsResponse> {
        let mut r = self.http_client.client.get("/v2/cards");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_id {
            r = r.query("customer_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.include_disabled {
            r = r.query("include_disabled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reference_id {
            r = r.query("reference_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_order {
            r = r.query("sort_order", &unwrapped.to_string());
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
    pub fn include_disabled(mut self, include_disabled: bool) -> Self {
        self.include_disabled = Some(include_disabled);
        self
    }
    pub fn reference_id(mut self, reference_id: &str) -> Self {
        self.reference_id = Some(reference_id.to_owned());
        self
    }
    pub fn sort_order(mut self, sort_order: &str) -> Self {
        self.sort_order = Some(sort_order.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCardsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListCardsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}