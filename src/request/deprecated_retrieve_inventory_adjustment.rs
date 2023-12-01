use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeprecatedRetrieveInventoryAdjustmentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub adjustment_id: String,
}
impl<'a> DeprecatedRetrieveInventoryAdjustmentRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveInventoryAdjustmentResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/inventory/adjustment/{adjustment_id}", adjustment_id = self
                    .adjustment_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeprecatedRetrieveInventoryAdjustmentRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveInventoryAdjustmentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}