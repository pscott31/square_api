use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteCustomerCustomAttributeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub customer_id: String,
    pub key: String,
}
impl<'a> DeleteCustomerCustomAttributeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeleteCustomerCustomAttributeResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/customers/{customer_id}/custom-attributes/{key}", customer_id =
                    self.customer_id, key = self.key
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteCustomerCustomAttributeRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteCustomerCustomAttributeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}