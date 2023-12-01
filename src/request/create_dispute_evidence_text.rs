use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateDisputeEvidenceTextRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub dispute_id: String,
    pub evidence_text: String,
    pub evidence_type: Option<String>,
    pub idempotency_key: String,
}
impl<'a> CreateDisputeEvidenceTextRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateDisputeEvidenceTextResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/disputes/{dispute_id}/evidence-text", dispute_id = self
                    .dispute_id
                ),
            );
        r = r.json(json!({ "evidence_text" : self.evidence_text }));
        if let Some(ref unwrapped) = self.evidence_type {
            r = r.json(json!({ "evidence_type" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn evidence_type(mut self, evidence_type: &str) -> Self {
        self.evidence_type = Some(evidence_type.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateDisputeEvidenceTextRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateDisputeEvidenceTextResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}