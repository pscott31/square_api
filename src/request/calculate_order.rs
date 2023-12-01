use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CalculateOrderRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub order: Order,
    pub proposed_rewards: Option<Vec<OrderReward>>,
}
impl<'a> CalculateOrderRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CalculateOrderResponse> {
        let mut r = self.http_client.client.post("/v2/orders/calculate");
        r = r.json(json!({ "order" : self.order }));
        if let Some(ref unwrapped) = self.proposed_rewards {
            r = r.json(json!({ "proposed_rewards" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn proposed_rewards(mut self, proposed_rewards: Vec<OrderReward>) -> Self {
        self.proposed_rewards = Some(proposed_rewards);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CalculateOrderRequest<'a> {
    type Output = httpclient::InMemoryResult<CalculateOrderResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}