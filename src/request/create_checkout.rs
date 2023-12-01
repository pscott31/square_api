use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCheckoutRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub additional_recipients: Option<Vec<ChargeRequestAdditionalRecipient>>,
    pub ask_for_shipping_address: Option<bool>,
    pub idempotency_key: String,
    pub location_id: String,
    pub merchant_support_email: Option<String>,
    pub note: Option<String>,
    pub order: CreateOrderRequest,
    pub pre_populate_buyer_email: Option<String>,
    pub pre_populate_shipping_address: Option<Address>,
    pub redirect_url: Option<String>,
}
impl<'a> CreateCheckoutRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateCheckoutResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/locations/{location_id}/checkouts", location_id = self
                    .location_id
                ),
            );
        if let Some(ref unwrapped) = self.additional_recipients {
            r = r.json(json!({ "additional_recipients" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ask_for_shipping_address {
            r = r.json(json!({ "ask_for_shipping_address" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.merchant_support_email {
            r = r.json(json!({ "merchant_support_email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.note {
            r = r.json(json!({ "note" : unwrapped }));
        }
        r = r.json(json!({ "order" : self.order }));
        if let Some(ref unwrapped) = self.pre_populate_buyer_email {
            r = r.json(json!({ "pre_populate_buyer_email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.pre_populate_shipping_address {
            r = r.json(json!({ "pre_populate_shipping_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.redirect_url {
            r = r.json(json!({ "redirect_url" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn additional_recipients(
        mut self,
        additional_recipients: Vec<ChargeRequestAdditionalRecipient>,
    ) -> Self {
        self.additional_recipients = Some(additional_recipients);
        self
    }
    pub fn ask_for_shipping_address(mut self, ask_for_shipping_address: bool) -> Self {
        self.ask_for_shipping_address = Some(ask_for_shipping_address);
        self
    }
    pub fn merchant_support_email(mut self, merchant_support_email: &str) -> Self {
        self.merchant_support_email = Some(merchant_support_email.to_owned());
        self
    }
    pub fn note(mut self, note: &str) -> Self {
        self.note = Some(note.to_owned());
        self
    }
    pub fn pre_populate_buyer_email(mut self, pre_populate_buyer_email: &str) -> Self {
        self.pre_populate_buyer_email = Some(pre_populate_buyer_email.to_owned());
        self
    }
    pub fn pre_populate_shipping_address(
        mut self,
        pre_populate_shipping_address: Address,
    ) -> Self {
        self.pre_populate_shipping_address = Some(pre_populate_shipping_address);
        self
    }
    pub fn redirect_url(mut self, redirect_url: &str) -> Self {
        self.redirect_url = Some(redirect_url.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCheckoutRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateCheckoutResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}