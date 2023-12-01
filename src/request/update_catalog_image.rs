use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateCatalogImageRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub image_id: String,
}
impl<'a> UpdateCatalogImageRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateCatalogImageResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/catalog/images/{image_id}", image_id = self.image_id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateCatalogImageRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateCatalogImageResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}