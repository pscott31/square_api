use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RefundPaymentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub amount_money: Money,
    pub app_fee_money: Option<Money>,
    pub idempotency_key: String,
    pub payment_id: Option<String>,
    pub payment_version_token: Option<String>,
    pub reason: Option<String>,
    pub team_member_id: Option<String>,
}
impl<'a> RefundPaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<RefundPaymentResponse> {
        let mut r = self.http_client.client.post("/v2/refunds");
        r = r.json(json!({ "amount_money" : self.amount_money }));
        if let Some(ref unwrapped) = self.app_fee_money {
            r = r.json(json!({ "app_fee_money" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.payment_id {
            r = r.json(json!({ "payment_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_version_token {
            r = r.json(json!({ "payment_version_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reason {
            r = r.json(json!({ "reason" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.team_member_id {
            r = r.json(json!({ "team_member_id" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn app_fee_money(mut self, app_fee_money: Money) -> Self {
        self.app_fee_money = Some(app_fee_money);
        self
    }
    pub fn payment_id(mut self, payment_id: &str) -> Self {
        self.payment_id = Some(payment_id.to_owned());
        self
    }
    pub fn payment_version_token(mut self, payment_version_token: &str) -> Self {
        self.payment_version_token = Some(payment_version_token.to_owned());
        self
    }
    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }
    pub fn team_member_id(mut self, team_member_id: &str) -> Self {
        self.team_member_id = Some(team_member_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for RefundPaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<RefundPaymentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}