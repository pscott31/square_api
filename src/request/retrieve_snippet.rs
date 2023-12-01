use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveSnippetRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub site_id: String,
}
impl<'a> RetrieveSnippetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<RetrieveSnippetResponse> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v2/sites/{site_id}/snippet", site_id = self.site_id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveSnippetRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveSnippetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}