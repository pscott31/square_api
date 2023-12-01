use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreatePaymentRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub accept_partial_authorization: Option<bool>,
    pub amount_money: Money,
    pub app_fee_money: Option<Money>,
    pub autocomplete: Option<bool>,
    pub billing_address: Option<Address>,
    pub buyer_email_address: Option<String>,
    pub cash_details: Option<CashPaymentDetails>,
    pub customer_id: Option<String>,
    pub delay_action: Option<String>,
    pub delay_duration: Option<String>,
    pub external_details: Option<ExternalPaymentDetails>,
    pub idempotency_key: String,
    pub location_id: Option<String>,
    pub note: Option<String>,
    pub order_id: Option<String>,
    pub reference_id: Option<String>,
    pub shipping_address: Option<Address>,
    pub source_id: String,
    pub statement_description_identifier: Option<String>,
    pub team_member_id: Option<String>,
    pub tip_money: Option<Money>,
    pub verification_token: Option<String>,
}
impl<'a> CreatePaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreatePaymentResponse> {
        let mut r = self.http_client.client.post("/v2/payments");
        if let Some(ref unwrapped) = self.accept_partial_authorization {
            r = r.json(json!({ "accept_partial_authorization" : unwrapped }));
        }
        r = r.json(json!({ "amount_money" : self.amount_money }));
        if let Some(ref unwrapped) = self.app_fee_money {
            r = r.json(json!({ "app_fee_money" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.autocomplete {
            r = r.json(json!({ "autocomplete" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_address {
            r = r.json(json!({ "billing_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.buyer_email_address {
            r = r.json(json!({ "buyer_email_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cash_details {
            r = r.json(json!({ "cash_details" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer_id {
            r = r.json(json!({ "customer_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.delay_action {
            r = r.json(json!({ "delay_action" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.delay_duration {
            r = r.json(json!({ "delay_duration" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_details {
            r = r.json(json!({ "external_details" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.location_id {
            r = r.json(json!({ "location_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.note {
            r = r.json(json!({ "note" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.order_id {
            r = r.json(json!({ "order_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reference_id {
            r = r.json(json!({ "reference_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.shipping_address {
            r = r.json(json!({ "shipping_address" : unwrapped }));
        }
        r = r.json(json!({ "source_id" : self.source_id }));
        if let Some(ref unwrapped) = self.statement_description_identifier {
            r = r.json(json!({ "statement_description_identifier" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.team_member_id {
            r = r.json(json!({ "team_member_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tip_money {
            r = r.json(json!({ "tip_money" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.verification_token {
            r = r.json(json!({ "verification_token" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn accept_partial_authorization(
        mut self,
        accept_partial_authorization: bool,
    ) -> Self {
        self.accept_partial_authorization = Some(accept_partial_authorization);
        self
    }
    pub fn app_fee_money(mut self, app_fee_money: Money) -> Self {
        self.app_fee_money = Some(app_fee_money);
        self
    }
    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }
    pub fn billing_address(mut self, billing_address: Address) -> Self {
        self.billing_address = Some(billing_address);
        self
    }
    pub fn buyer_email_address(mut self, buyer_email_address: &str) -> Self {
        self.buyer_email_address = Some(buyer_email_address.to_owned());
        self
    }
    pub fn cash_details(mut self, cash_details: CashPaymentDetails) -> Self {
        self.cash_details = Some(cash_details);
        self
    }
    pub fn customer_id(mut self, customer_id: &str) -> Self {
        self.customer_id = Some(customer_id.to_owned());
        self
    }
    pub fn delay_action(mut self, delay_action: &str) -> Self {
        self.delay_action = Some(delay_action.to_owned());
        self
    }
    pub fn delay_duration(mut self, delay_duration: &str) -> Self {
        self.delay_duration = Some(delay_duration.to_owned());
        self
    }
    pub fn external_details(mut self, external_details: ExternalPaymentDetails) -> Self {
        self.external_details = Some(external_details);
        self
    }
    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_owned());
        self
    }
    pub fn note(mut self, note: &str) -> Self {
        self.note = Some(note.to_owned());
        self
    }
    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_owned());
        self
    }
    pub fn reference_id(mut self, reference_id: &str) -> Self {
        self.reference_id = Some(reference_id.to_owned());
        self
    }
    pub fn shipping_address(mut self, shipping_address: Address) -> Self {
        self.shipping_address = Some(shipping_address);
        self
    }
    pub fn statement_description_identifier(
        mut self,
        statement_description_identifier: &str,
    ) -> Self {
        self
            .statement_description_identifier = Some(
            statement_description_identifier.to_owned(),
        );
        self
    }
    pub fn team_member_id(mut self, team_member_id: &str) -> Self {
        self.team_member_id = Some(team_member_id.to_owned());
        self
    }
    pub fn tip_money(mut self, tip_money: Money) -> Self {
        self.tip_money = Some(tip_money);
        self
    }
    pub fn verification_token(mut self, verification_token: &str) -> Self {
        self.verification_token = Some(verification_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreatePaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<CreatePaymentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}