use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListGiftCardActivitiesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub begin_time: Option<String>,
    pub cursor: Option<String>,
    pub end_time: Option<String>,
    pub gift_card_id: Option<String>,
    pub limit: Option<i64>,
    pub location_id: Option<String>,
    pub sort_order: Option<String>,
    pub type_: Option<String>,
}
impl<'a> ListGiftCardActivitiesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListGiftCardActivitiesResponse> {
        let mut r = self.http_client.client.get("/v2/gift-cards/activities");
        if let Some(ref unwrapped) = self.begin_time {
            r = r.query("begin_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.gift_card_id {
            r = r.query("gift_card_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_id {
            r = r.query("location_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_order {
            r = r.query("sort_order", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn begin_time(mut self, begin_time: &str) -> Self {
        self.begin_time = Some(begin_time.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_owned());
        self
    }
    pub fn gift_card_id(mut self, gift_card_id: &str) -> Self {
        self.gift_card_id = Some(gift_card_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_owned());
        self
    }
    pub fn sort_order(mut self, sort_order: &str) -> Self {
        self.sort_order = Some(sort_order.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListGiftCardActivitiesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListGiftCardActivitiesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}