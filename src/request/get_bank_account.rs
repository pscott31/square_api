use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetBankAccountRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub bank_account_id: String,
}
impl<'a> GetBankAccountRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<GetBankAccountResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/bank-accounts/{bank_account_id}", bank_account_id = self
                    .bank_account_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetBankAccountRequest<'a> {
    type Output = httpclient::InMemoryResult<GetBankAccountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}