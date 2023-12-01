use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreatePaymentLinkRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub checkout_options: Option<CheckoutOptions>,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
    pub order: Option<Order>,
    pub payment_note: Option<String>,
    pub pre_populated_data: Option<PrePopulatedData>,
    pub quick_pay: Option<QuickPay>,
}
impl<'a> CreatePaymentLinkRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreatePaymentLinkResponse> {
        let mut r = self.http_client.client.post("/v2/online-checkout/payment-links");
        if let Some(ref unwrapped) = self.checkout_options {
            r = r.json(json!({ "checkout_options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.description {
            r = r.json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order {
            r = r.json(json!({ "order" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_note {
            r = r.json(json!({ "payment_note" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.pre_populated_data {
            r = r.json(json!({ "pre_populated_data" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.quick_pay {
            r = r.json(json!({ "quick_pay" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn checkout_options(mut self, checkout_options: CheckoutOptions) -> Self {
        self.checkout_options = Some(checkout_options);
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }
    pub fn payment_note(mut self, payment_note: &str) -> Self {
        self.payment_note = Some(payment_note.to_owned());
        self
    }
    pub fn pre_populated_data(mut self, pre_populated_data: PrePopulatedData) -> Self {
        self.pre_populated_data = Some(pre_populated_data);
        self
    }
    pub fn quick_pay(mut self, quick_pay: QuickPay) -> Self {
        self.quick_pay = Some(quick_pay);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreatePaymentLinkRequest<'a> {
    type Output = httpclient::InMemoryResult<CreatePaymentLinkResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}