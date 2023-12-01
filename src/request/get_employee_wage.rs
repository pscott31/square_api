use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetEmployeeWageRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub id: String,
}
impl<'a> GetEmployeeWageRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<GetEmployeeWageResponse> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v2/labor/employee-wages/{id}", id = self.id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetEmployeeWageRequest<'a> {
    type Output = httpclient::InMemoryResult<GetEmployeeWageResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}