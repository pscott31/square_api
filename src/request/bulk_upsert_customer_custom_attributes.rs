use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BulkUpsertCustomerCustomAttributesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub values: serde_json::Value,
}
impl<'a> BulkUpsertCustomerCustomAttributesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BulkUpsertCustomerCustomAttributesResponse> {
        let mut r = self
            .http_client
            .client
            .post("/v2/customers/custom-attributes/bulk-upsert");
        r = r.json(json!({ "values" : self.values }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for BulkUpsertCustomerCustomAttributesRequest<'a> {
    type Output = httpclient::InMemoryResult<BulkUpsertCustomerCustomAttributesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}