use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateTeamMemberRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub team_member: Option<TeamMember>,
    pub team_member_id: String,
}
impl<'a> UpdateTeamMemberRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateTeamMemberResponse> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/v2/team-members/{team_member_id}", team_member_id = self
                    .team_member_id
                ),
            );
        if let Some(ref unwrapped) = self.team_member {
            r = r.json(json!({ "team_member" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn team_member(mut self, team_member: TeamMember) -> Self {
        self.team_member = Some(team_member);
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateTeamMemberRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateTeamMemberResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}