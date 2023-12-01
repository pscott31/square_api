use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkCustomerToGiftCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub customer_id: String,
    pub gift_card_id: String,
}
impl<'a> LinkCustomerToGiftCardRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<LinkCustomerToGiftCardResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/gift-cards/{gift_card_id}/link-customer", gift_card_id = self
                    .gift_card_id
                ),
            );
        r = r.json(json!({ "customer_id" : self.customer_id }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for LinkCustomerToGiftCardRequest<'a> {
    type Output = httpclient::InMemoryResult<LinkCustomerToGiftCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}