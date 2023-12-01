use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetBankAccountByV1IdRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub v1_bank_account_id: String,
}
impl<'a> GetBankAccountByV1IdRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<GetBankAccountByV1IdResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/bank-accounts/by-v1-id/{v1_bank_account_id}", v1_bank_account_id
                    = self.v1_bank_account_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetBankAccountByV1IdRequest<'a> {
    type Output = httpclient::InMemoryResult<GetBankAccountByV1IdResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}