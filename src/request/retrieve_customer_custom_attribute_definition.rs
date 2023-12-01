use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveCustomerCustomAttributeDefinitionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub key: String,
    pub version: Option<i64>,
}
impl<'a> RetrieveCustomerCustomAttributeDefinitionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        RetrieveCustomerCustomAttributeDefinitionResponse,
    > {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/customers/custom-attribute-definitions/{key}", key = self.key
                ),
            );
        if let Some(ref unwrapped) = self.version {
            r = r.query("version", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for RetrieveCustomerCustomAttributeDefinitionRequest<'a> {
    type Output = httpclient::InMemoryResult<
        RetrieveCustomerCustomAttributeDefinitionResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}