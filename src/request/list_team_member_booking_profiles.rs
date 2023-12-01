use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListTeamMemberBookingProfilesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub bookable_only: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub location_id: Option<String>,
}
impl<'a> ListTeamMemberBookingProfilesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListTeamMemberBookingProfilesResponse> {
        let mut r = self
            .http_client
            .client
            .get("/v2/bookings/team-member-booking-profiles");
        if let Some(ref unwrapped) = self.bookable_only {
            r = r.query("bookable_only", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_id {
            r = r.query("location_id", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn bookable_only(mut self, bookable_only: bool) -> Self {
        self.bookable_only = Some(bookable_only);
        self
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
}
impl<'a> ::std::future::IntoFuture for ListTeamMemberBookingProfilesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListTeamMemberBookingProfilesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}