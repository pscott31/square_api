use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateVendorRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: String,
    pub vendor: Option<Vendor>,
}
impl<'a> CreateVendorRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateVendorResponse> {
        let mut r = self.http_client.client.post("/v2/vendors/create");
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.vendor {
            r = r.json(json!({ "vendor" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn vendor(mut self, vendor: Vendor) -> Self {
        self.vendor = Some(vendor);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateVendorRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateVendorResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}