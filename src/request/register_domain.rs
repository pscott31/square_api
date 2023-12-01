use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RegisterDomainRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub domain_name: String,
}
impl<'a> RegisterDomainRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<RegisterDomainResponse> {
        let mut r = self.http_client.client.post("/v2/apple-pay/domains");
        r = r.json(json!({ "domain_name" : self.domain_name }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RegisterDomainRequest<'a> {
    type Output = httpclient::InMemoryResult<RegisterDomainResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}