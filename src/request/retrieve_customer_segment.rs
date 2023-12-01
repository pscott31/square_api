use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveCustomerSegmentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub segment_id: String,
}
impl<'a> RetrieveCustomerSegmentRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveCustomerSegmentResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/customers/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveCustomerSegmentRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveCustomerSegmentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}