use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateWorkweekConfigRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub id: String,
    pub workweek_config: WorkweekConfig,
}
impl<'a> UpdateWorkweekConfigRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpdateWorkweekConfigResponse> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/v2/labor/workweek-configs/{id}", id = self.id));
        r = r.json(json!({ "workweek_config" : self.workweek_config }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateWorkweekConfigRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateWorkweekConfigResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}