use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateCustomerCustomAttributeDefinitionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub custom_attribute_definition: CustomAttributeDefinition,
    pub idempotency_key: Option<String>,
    pub key: String,
}
impl<'a> UpdateCustomerCustomAttributeDefinitionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpdateCustomerCustomAttributeDefinitionResponse> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/v2/customers/custom-attribute-definitions/{key}", key = self.key
                ),
            );
        r = r
            .json(
                json!(
                    { "custom_attribute_definition" : self.custom_attribute_definition }
                ),
            );
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
impl<'a> ::std::future::IntoFuture
for UpdateCustomerCustomAttributeDefinitionRequest<'a> {
    type Output = httpclient::InMemoryResult<
        UpdateCustomerCustomAttributeDefinitionResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}