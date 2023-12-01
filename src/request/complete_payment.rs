use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CompletePaymentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub payment_id: String,
    pub version_token: Option<String>,
}
impl<'a> CompletePaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CompletePaymentResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/payments/{payment_id}/complete", payment_id = self.payment_id
                ),
            );
        if let Some(ref unwrapped) = self.version_token {
            r = r.json(json!({ "version_token" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn version_token(mut self, version_token: &str) -> Self {
        self.version_token = Some(version_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CompletePaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<CompletePaymentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}