use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTeamMemberRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub idempotency_key: Option<String>,
    pub team_member: Option<TeamMember>,
}
impl<'a> CreateTeamMemberRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateTeamMemberResponse> {
        let mut r = self.http_client.client.post("/v2/team-members");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.team_member {
            r = r.json(json!({ "team_member" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn team_member(mut self, team_member: TeamMember) -> Self {
        self.team_member = Some(team_member);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateTeamMemberRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTeamMemberResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}