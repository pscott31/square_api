use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveWageSettingRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub team_member_id: String,
}
impl<'a> RetrieveWageSettingRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<RetrieveWageSettingResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/team-members/{team_member_id}/wage-setting", team_member_id =
                    self.team_member_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveWageSettingRequest<'a> {
    type Output = httpclient::InMemoryResult<RetrieveWageSettingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}