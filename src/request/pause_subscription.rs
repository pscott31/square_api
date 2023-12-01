use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PauseSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub pause_cycle_duration: Option<i64>,
    pub pause_effective_date: Option<String>,
    pub pause_reason: Option<String>,
    pub resume_change_timing: Option<String>,
    pub resume_effective_date: Option<String>,
    pub subscription_id: String,
}
impl<'a> PauseSubscriptionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<PauseSubscriptionResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/subscriptions/{subscription_id}/pause", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.pause_cycle_duration {
            r = r.json(json!({ "pause_cycle_duration" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.pause_effective_date {
            r = r.json(json!({ "pause_effective_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.pause_reason {
            r = r.json(json!({ "pause_reason" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.resume_change_timing {
            r = r.json(json!({ "resume_change_timing" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.resume_effective_date {
            r = r.json(json!({ "resume_effective_date" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn pause_cycle_duration(mut self, pause_cycle_duration: i64) -> Self {
        self.pause_cycle_duration = Some(pause_cycle_duration);
        self
    }
    pub fn pause_effective_date(mut self, pause_effective_date: &str) -> Self {
        self.pause_effective_date = Some(pause_effective_date.to_owned());
        self
    }
    pub fn pause_reason(mut self, pause_reason: &str) -> Self {
        self.pause_reason = Some(pause_reason.to_owned());
        self
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
impl<'a> ::std::future::IntoFuture for PauseSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<PauseSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}