use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateCustomerRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub address: Option<Address>,
    pub birthday: Option<String>,
    pub company_name: Option<String>,
    pub customer_id: String,
    pub email_address: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub nickname: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub reference_id: Option<String>,
    pub tax_ids: Option<CustomerTaxIds>,
    pub version: Option<i64>,
}
impl<'a> UpdateCustomerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateCustomerResponse> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!("/v2/customers/{customer_id}", customer_id = self.customer_id),
            );
        if let Some(ref unwrapped) = self.address {
            r = r.json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.birthday {
            r = r.json(json!({ "birthday" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.company_name {
            r = r.json(json!({ "company_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email_address {
            r = r.json(json!({ "email_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.family_name {
            r = r.json(json!({ "family_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.given_name {
            r = r.json(json!({ "given_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.nickname {
            r = r.json(json!({ "nickname" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.note {
            r = r.json(json!({ "note" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.phone_number {
            r = r.json(json!({ "phone_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reference_id {
            r = r.json(json!({ "reference_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_ids {
            r = r.json(json!({ "tax_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.version {
            r = r.json(json!({ "version" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }
    pub fn birthday(mut self, birthday: &str) -> Self {
        self.birthday = Some(birthday.to_owned());
        self
    }
    pub fn company_name(mut self, company_name: &str) -> Self {
        self.company_name = Some(company_name.to_owned());
        self
    }
    pub fn email_address(mut self, email_address: &str) -> Self {
        self.email_address = Some(email_address.to_owned());
        self
    }
    pub fn family_name(mut self, family_name: &str) -> Self {
        self.family_name = Some(family_name.to_owned());
        self
    }
    pub fn given_name(mut self, given_name: &str) -> Self {
        self.given_name = Some(given_name.to_owned());
        self
    }
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_owned());
        self
    }
    pub fn note(mut self, note: &str) -> Self {
        self.note = Some(note.to_owned());
        self
    }
    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.phone_number = Some(phone_number.to_owned());
        self
    }
    pub fn reference_id(mut self, reference_id: &str) -> Self {
        self.reference_id = Some(reference_id.to_owned());
        self
    }
    pub fn tax_ids(mut self, tax_ids: CustomerTaxIds) -> Self {
        self.tax_ids = Some(tax_ids);
        self
    }
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateCustomerRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateCustomerResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}