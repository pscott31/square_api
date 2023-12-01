use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RemoveGroupFromCustomerRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub customer_id: String,
    pub group_id: String,
}
impl<'a> RemoveGroupFromCustomerRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RemoveGroupFromCustomerResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/customers/{customer_id}/groups/{group_id}", customer_id = self
                    .customer_id, group_id = self.group_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RemoveGroupFromCustomerRequest<'a> {
    type Output = httpclient::InMemoryResult<RemoveGroupFromCustomerResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}