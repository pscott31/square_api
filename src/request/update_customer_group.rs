use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateCustomerGroupRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub group: CustomerGroup,
    pub group_id: String,
}
impl<'a> UpdateCustomerGroupRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpdateCustomerGroupResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/customers/groups/{group_id}", group_id = self.group_id));
        r = r.json(json!({ "group" : self.group }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateCustomerGroupRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateCustomerGroupResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}