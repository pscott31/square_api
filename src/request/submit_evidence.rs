use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SubmitEvidenceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub dispute_id: String,
}
impl<'a> SubmitEvidenceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SubmitEvidenceResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/disputes/{dispute_id}/submit-evidence", dispute_id = self
                    .dispute_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for SubmitEvidenceRequest<'a> {
    type Output = httpclient::InMemoryResult<SubmitEvidenceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}