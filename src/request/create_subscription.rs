use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateSubscriptionRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub canceled_date: Option<String>,
    pub card_id: Option<String>,
    pub customer_id: String,
    pub idempotency_key: Option<String>,
    pub location_id: String,
    pub plan_id: String,
    pub price_override_money: Option<Money>,
    pub source: Option<SubscriptionSource>,
    pub start_date: Option<String>,
    pub tax_percentage: Option<String>,
    pub timezone: Option<String>,
}
impl<'a> CreateSubscriptionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateSubscriptionResponse> {
        let mut r = self.http_client.client.post("/v2/subscriptions");
        if let Some(ref unwrapped) = self.canceled_date {
            r = r.json(json!({ "canceled_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.card_id {
            r = r.json(json!({ "card_id" : unwrapped }));
        }
        r = r.json(json!({ "customer_id" : self.customer_id }));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        r = r.json(json!({ "location_id" : self.location_id }));
        r = r.json(json!({ "plan_id" : self.plan_id }));
        if let Some(ref unwrapped) = self.price_override_money {
            r = r.json(json!({ "price_override_money" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.source {
            r = r.json(json!({ "source" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_percentage {
            r = r.json(json!({ "tax_percentage" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.json(json!({ "timezone" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn canceled_date(mut self, canceled_date: &str) -> Self {
        self.canceled_date = Some(canceled_date.to_owned());
        self
    }
    pub fn card_id(mut self, card_id: &str) -> Self {
        self.card_id = Some(card_id.to_owned());
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn price_override_money(mut self, price_override_money: Money) -> Self {
        self.price_override_money = Some(price_override_money);
        self
    }
    pub fn source(mut self, source: SubscriptionSource) -> Self {
        self.source = Some(source);
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn tax_percentage(mut self, tax_percentage: &str) -> Self {
        self.tax_percentage = Some(tax_percentage.to_owned());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateSubscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}