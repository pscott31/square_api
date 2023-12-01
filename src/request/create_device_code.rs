use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateDeviceCodeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub device_code: DeviceCode,
    pub idempotency_key: String,
}
impl<'a> CreateDeviceCodeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateDeviceCodeResponse> {
        let mut r = self.http_client.client.post("/v2/devices/codes");
        r = r.json(json!({ "device_code" : self.device_code }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CreateDeviceCodeRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateDeviceCodeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}