use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListDisputeEvidenceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub dispute_id: String,
}
impl<'a> ListDisputeEvidenceRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ListDisputeEvidenceResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v2/disputes/{dispute_id}/evidence", dispute_id = self.dispute_id
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListDisputeEvidenceRequest<'a> {
    type Output = httpclient::InMemoryResult<ListDisputeEvidenceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}