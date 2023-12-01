use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdatePaymentLinkRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub id: String,
    pub payment_link: PaymentLink,
}
impl<'a> UpdatePaymentLinkRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdatePaymentLinkResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/online-checkout/payment-links/{id}", id = self.id));
        r = r.json(json!({ "payment_link" : self.payment_link }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdatePaymentLinkRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdatePaymentLinkResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}