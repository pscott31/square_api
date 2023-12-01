use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BulkCreateVendorsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub vendors: serde_json::Value,
}
impl<'a> BulkCreateVendorsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<BulkCreateVendorsResponse> {
        let mut r = self.http_client.client.post("/v2/vendors/bulk-create");
        r = r.json(json!({ "vendors" : self.vendors }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for BulkCreateVendorsRequest<'a> {
    type Output = httpclient::InMemoryResult<BulkCreateVendorsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}