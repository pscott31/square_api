use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListLoyaltyPromotionsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub program_id: String,
    pub status: Option<String>,
}
impl<'a> ListLoyaltyPromotionsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListLoyaltyPromotionsResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/loyalty/programs/{program_id}/promotions", program_id = self
                    .program_id
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
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
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListLoyaltyPromotionsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListLoyaltyPromotionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}