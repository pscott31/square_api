use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveCashDrawerShiftRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub location_id: String,
    pub shift_id: String,
}
impl<'a> RetrieveCashDrawerShiftRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveCashDrawerShiftResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/v2/cash-drawers/shifts/{shift_id}", shift_id = self.shift_id),
            );
        r = r.query("location_id", &self.location_id.to_string());
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveCashDrawerShiftRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveCashDrawerShiftResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}