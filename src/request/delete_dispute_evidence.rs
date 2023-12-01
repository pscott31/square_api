use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteDisputeEvidenceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub dispute_id: String,
    pub evidence_id: String,
}
impl<'a> DeleteDisputeEvidenceRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeleteDisputeEvidenceResponse> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v2/disputes/{dispute_id}/evidence/{evidence_id}", dispute_id = self
                    .dispute_id, evidence_id = self.evidence_id
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteDisputeEvidenceRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteDisputeEvidenceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}