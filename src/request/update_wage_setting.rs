use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateWageSettingRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub team_member_id: String,
    pub wage_setting: WageSetting,
}
impl<'a> UpdateWageSettingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateWageSettingResponse> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/v2/team-members/{team_member_id}/wage-setting", team_member_id =
                    self.team_member_id
                ),
            );
        r = r.json(json!({ "wage_setting" : self.wage_setting }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateWageSettingRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateWageSettingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}