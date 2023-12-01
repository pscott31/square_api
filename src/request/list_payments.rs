use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListPaymentsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub begin_time: Option<String>,
    pub card_brand: Option<String>,
    pub cursor: Option<String>,
    pub end_time: Option<String>,
    pub last4: Option<String>,
    pub limit: Option<i64>,
    pub location_id: Option<String>,
    pub sort_order: Option<String>,
    pub total: Option<i64>,
}
impl<'a> ListPaymentsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListPaymentsResponse> {
        let mut r = self.http_client.client.get("/v2/payments");
        if let Some(ref unwrapped) = self.begin_time {
            r = r.query("begin_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.card_brand {
            r = r.query("card_brand", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.last4 {
            r = r.query("last_4", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.total {
            r = r.query("total", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn begin_time(mut self, begin_time: &str) -> Self {
        self.begin_time = Some(begin_time.to_owned());
        self
    }
    pub fn card_brand(mut self, card_brand: &str) -> Self {
        self.card_brand = Some(card_brand.to_owned());
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
    pub fn last4(mut self, last4: &str) -> Self {
        self.last4 = Some(last4.to_owned());
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
    pub fn total(mut self, total: i64) -> Self {
        self.total = Some(total);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListPaymentsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListPaymentsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}