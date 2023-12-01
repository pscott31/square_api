use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ResumeSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub resume_change_timing: Option<String>,
    pub resume_effective_date: Option<String>,
    pub subscription_id: String,
}
impl<'a> ResumeSubscriptionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ResumeSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/subscriptions/{subscription_id}/resume", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.resume_change_timing {
            r = r.json(json!({ "resume_change_timing" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.resume_effective_date {
            r = r.json(json!({ "resume_effective_date" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn resume_change_timing(mut self, resume_change_timing: &str) -> Self {
        self.resume_change_timing = Some(resume_change_timing.to_owned());
        self
    }
    pub fn resume_effective_date(mut self, resume_effective_date: &str) -> Self {
        self.resume_effective_date = Some(resume_effective_date.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ResumeSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<ResumeSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}