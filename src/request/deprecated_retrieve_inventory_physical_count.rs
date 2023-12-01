use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeprecatedRetrieveInventoryPhysicalCountRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub physical_count_id: String,
}
impl<'a> DeprecatedRetrieveInventoryPhysicalCountRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveInventoryPhysicalCountResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/inventory/physical-count/{physical_count_id}", physical_count_id
                    = self.physical_count_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for DeprecatedRetrieveInventoryPhysicalCountRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveInventoryPhysicalCountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}