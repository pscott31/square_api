use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelBookingRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub booking_id: String,
    pub booking_version: Option<i64>,
    pub idempotency_key: Option<String>,
}
impl<'a> CancelBookingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CancelBookingResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/bookings/{booking_id}/cancel", booking_id = self.booking_id
                ),
            );
        if let Some(ref unwrapped) = self.booking_version {
            r = r.json(json!({ "booking_version" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn booking_version(mut self, booking_version: i64) -> Self {
        self.booking_version = Some(booking_version);
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CancelBookingRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelBookingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}