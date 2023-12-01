use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCustomerCustomAttributesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub customer_id: String,
    pub limit: Option<i64>,
    pub with_definitions: Option<bool>,
}
impl<'a> ListCustomerCustomAttributesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListCustomerCustomAttributesResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/customers/{customer_id}/custom-attributes", customer_id = self
                    .customer_id
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.with_definitions {
            r = r.query("with_definitions", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_definitions(mut self, with_definitions: bool) -> Self {
        self.with_definitions = Some(with_definitions);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCustomerCustomAttributesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListCustomerCustomAttributesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}