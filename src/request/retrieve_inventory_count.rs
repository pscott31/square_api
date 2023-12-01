use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveInventoryCountRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub catalog_object_id: String,
    pub cursor: Option<String>,
    pub location_ids: Option<String>,
}
impl<'a> RetrieveInventoryCountRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveInventoryCountResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/inventory/{catalog_object_id}", catalog_object_id = self
                    .catalog_object_id
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location_ids {
            r = r.query("location_ids", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn location_ids(mut self, location_ids: &str) -> Self {
        self.location_ids = Some(location_ids.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveInventoryCountRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveInventoryCountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}