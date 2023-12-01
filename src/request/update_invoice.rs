use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateInvoiceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub fields_to_clear: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
    pub invoice: Invoice,
    pub invoice_id: String,
}
impl<'a> UpdateInvoiceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<UpdateInvoiceResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/invoices/{invoice_id}", invoice_id = self.invoice_id));
        if let Some(ref unwrapped) = self.fields_to_clear {
            r = r.json(json!({ "fields_to_clear" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        r = r.json(json!({ "invoice" : self.invoice }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn fields_to_clear(
        mut self,
        fields_to_clear: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .fields_to_clear = Some(
            fields_to_clear.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateInvoiceRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateInvoiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}