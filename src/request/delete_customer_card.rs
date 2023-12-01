use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteCustomerCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub card_id: String,
    pub customer_id: String,
}
impl<'a> DeleteCustomerCardRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeleteCustomerCardResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/customers/{customer_id}/cards/{card_id}", card_id = self
                    .card_id, customer_id = self.customer_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteCustomerCardRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteCustomerCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}