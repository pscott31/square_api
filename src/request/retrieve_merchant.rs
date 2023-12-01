use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveMerchantRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub merchant_id: String,
}
impl<'a> RetrieveMerchantRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<RetrieveMerchantResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/v2/merchants/{merchant_id}", merchant_id = self.merchant_id),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveMerchantRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveMerchantResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}