use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct VoidTransactionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub location_id: String,
    pub transaction_id: String,
}
impl<'a> VoidTransactionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<VoidTransactionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/locations/{location_id}/transactions/{transaction_id}/void",
                    location_id = self.location_id, transaction_id = self.transaction_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for VoidTransactionRequest<'a> {
    type Output = httpclient::InMemoryResult<VoidTransactionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}