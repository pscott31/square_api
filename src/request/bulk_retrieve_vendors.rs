use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BulkRetrieveVendorsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub vendor_ids: Option<Vec<String>>,
}
impl<'a> BulkRetrieveVendorsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BulkRetrieveVendorsResponse> {
        let mut r = self.http_client.client.post("/v2/vendors/bulk-retrieve");
        if let Some(ref unwrapped) = self.vendor_ids {
            r = r.json(json!({ "vendor_ids" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn vendor_ids(
        mut self,
        vendor_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .vendor_ids = Some(
            vendor_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for BulkRetrieveVendorsRequest<'a> {
    type Output = httpclient::InMemoryResult<BulkRetrieveVendorsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}