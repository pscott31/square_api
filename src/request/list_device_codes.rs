use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListDeviceCodesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub location_id: Option<String>,
    pub product_type: Option<String>,
    pub status: Option<String>,
}
impl<'a> ListDeviceCodesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListDeviceCodesResponse> {
        let mut r = self.http_client.client.get("/v2/devices/codes");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_id {
            r = r.query("location_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.product_type {
            r = r.query("product_type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_owned());
        self
    }
    pub fn product_type(mut self, product_type: &str) -> Self {
        self.product_type = Some(product_type.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListDeviceCodesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListDeviceCodesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}