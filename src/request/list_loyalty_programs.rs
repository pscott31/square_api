use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListLoyaltyProgramsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
}
impl<'a> ListLoyaltyProgramsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListLoyaltyProgramsResponse> {
        let mut r = self.http_client.client.get("/v2/loyalty/programs");
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ListLoyaltyProgramsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListLoyaltyProgramsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}