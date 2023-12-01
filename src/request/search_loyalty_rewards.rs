use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchLoyaltyRewardsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub query: Option<SearchLoyaltyRewardsRequestLoyaltyRewardQuery>,
}
impl<'a> SearchLoyaltyRewardsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SearchLoyaltyRewardsResponse> {
        let mut r = self.http_client.client.post("/v2/loyalty/rewards/search");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query {
            r = r.json(json!({ "query" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn query(
        mut self,
        query: SearchLoyaltyRewardsRequestLoyaltyRewardQuery,
    ) -> Self {
        self.query = Some(query);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchLoyaltyRewardsRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchLoyaltyRewardsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}