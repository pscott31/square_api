use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BulkCreateTeamMembersRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub team_members: serde_json::Value,
}
impl<'a> BulkCreateTeamMembersRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BulkCreateTeamMembersResponse> {
        let mut r = self.http_client.client.post("/v2/team-members/bulk-create");
        r = r.json(json!({ "team_members" : self.team_members }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for BulkCreateTeamMembersRequest<'a> {
    type Output = httpclient::InMemoryResult<BulkCreateTeamMembersResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}