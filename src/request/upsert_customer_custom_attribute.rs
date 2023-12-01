use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpsertCustomerCustomAttributeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub custom_attribute: CustomAttribute,
    pub customer_id: String,
    pub idempotency_key: Option<String>,
    pub key: String,
}
impl<'a> UpsertCustomerCustomAttributeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpsertCustomerCustomAttributeResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/customers/{customer_id}/custom-attributes/{key}", customer_id =
                    self.customer_id, key = self.key
                ),
            );
        r = r.json(json!({ "custom_attribute" : self.custom_attribute }));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpsertCustomerCustomAttributeRequest<'a> {
    type Output = httpclient::InMemoryResult<UpsertCustomerCustomAttributeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}