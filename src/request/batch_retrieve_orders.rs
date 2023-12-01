use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BatchRetrieveOrdersRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub location_id: Option<String>,
    pub order_ids: Vec<String>,
}
impl<'a> BatchRetrieveOrdersRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BatchRetrieveOrdersResponse> {
        let mut r = self.http_client.client.post("/v2/orders/batch-retrieve");
        if let Some(ref unwrapped) = self.location_id {
            r = r.json(json!({ "location_id" : unwrapped }));
        }
        r = r.json(json!({ "order_ids" : self.order_ids }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for BatchRetrieveOrdersRequest<'a> {
    type Output = httpclient::InMemoryResult<BatchRetrieveOrdersResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}