use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCustomerCardRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub billing_address: Option<Address>,
    pub card_nonce: String,
    pub cardholder_name: Option<String>,
    pub customer_id: String,
    pub verification_token: Option<String>,
}
impl<'a> CreateCustomerCardRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateCustomerCardResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/customers/{customer_id}/cards", customer_id = self.customer_id
                ),
            );
        if let Some(ref unwrapped) = self.billing_address {
            r = r.json(json!({ "billing_address" : unwrapped }));
        }
        r = r.json(json!({ "card_nonce" : self.card_nonce }));
        if let Some(ref unwrapped) = self.cardholder_name {
            r = r.json(json!({ "cardholder_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.verification_token {
            r = r.json(json!({ "verification_token" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn billing_address(mut self, billing_address: Address) -> Self {
        self.billing_address = Some(billing_address);
        self
    }
    pub fn cardholder_name(mut self, cardholder_name: &str) -> Self {
        self.cardholder_name = Some(cardholder_name.to_owned());
        self
    }
    pub fn verification_token(mut self, verification_token: &str) -> Self {
        self.verification_token = Some(verification_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCustomerCardRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateCustomerCardResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}