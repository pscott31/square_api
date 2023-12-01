use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelInvoiceRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub invoice_id: String,
    pub version: i64,
}
impl<'a> CancelInvoiceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CancelInvoiceResponse> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v2/invoices/{invoice_id}/cancel", invoice_id = self.invoice_id
                ),
            );
        r = r.json(json!({ "version" : self.version }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for CancelInvoiceRequest<'a> {
    type Output = httpclient::InMemoryResult<CancelInvoiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}