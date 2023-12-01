use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveCustomerCustomAttributeRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub customer_id: String,
    pub key: String,
    pub version: Option<i64>,
    pub with_definition: Option<bool>,
}
impl<'a> RetrieveCustomerCustomAttributeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveCustomerCustomAttributeResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/customers/{customer_id}/custom-attributes/{key}", customer_id =
                    self.customer_id, key = self.key
                ),
            );
        if let Some(ref unwrapped) = self.version {
            r = r.query("version", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.with_definition {
            r = r.query("with_definition", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    pub fn with_definition(mut self, with_definition: bool) -> Self {
        self.with_definition = Some(with_definition);
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveCustomerCustomAttributeRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveCustomerCustomAttributeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}