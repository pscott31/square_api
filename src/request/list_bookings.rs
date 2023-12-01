use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListBookingsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub location_id: Option<String>,
    pub start_at_max: Option<String>,
    pub start_at_min: Option<String>,
    pub team_member_id: Option<String>,
}
impl<'a> ListBookingsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListBookingsResponse> {
        let mut r = self.http_client.client.get("/v2/bookings");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_id {
            r = r.query("location_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_at_max {
            r = r.query("start_at_max", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_at_min {
            r = r.query("start_at_min", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.team_member_id {
            r = r.query("team_member_id", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
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
    pub fn start_at_max(mut self, start_at_max: &str) -> Self {
        self.start_at_max = Some(start_at_max.to_owned());
        self
    }
    pub fn start_at_min(mut self, start_at_min: &str) -> Self {
        self.start_at_min = Some(start_at_min.to_owned());
        self
    }
    pub fn team_member_id(mut self, team_member_id: &str) -> Self {
        self.team_member_id = Some(team_member_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListBookingsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListBookingsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}