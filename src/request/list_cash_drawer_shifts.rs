use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCashDrawerShiftsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub begin_time: Option<String>,
    pub cursor: Option<String>,
    pub end_time: Option<String>,
    pub limit: Option<i64>,
    pub location_id: String,
    pub sort_order: Option<String>,
}
impl<'a> ListCashDrawerShiftsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListCashDrawerShiftsResponse> {
        let mut r = self.http_client.client.get("/v2/cash-drawers/shifts");
        if let Some(ref unwrapped) = self.begin_time {
            r = r.query("begin_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        r = r.query("location_id", &self.location_id.to_string());
        if let Some(ref unwrapped) = self.sort_order {
            r = r.query("sort_order", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn sort_order(mut self, sort_order: &str) -> Self {
        self.sort_order = Some(sort_order.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCashDrawerShiftsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListCashDrawerShiftsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}