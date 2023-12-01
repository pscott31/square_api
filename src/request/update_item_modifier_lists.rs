use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateItemModifierListsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub item_ids: Vec<String>,
    pub modifier_lists_to_disable: Option<Vec<String>>,
    pub modifier_lists_to_enable: Option<Vec<String>>,
}
impl<'a> UpdateItemModifierListsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<UpdateItemModifierListsResponse> {
        let mut r = self
            .http_client
            .client
            .post("/v2/catalog/update-item-modifier-lists");
        r = r.json(json!({ "item_ids" : self.item_ids }));
        if let Some(ref unwrapped) = self.modifier_lists_to_disable {
            r = r.json(json!({ "modifier_lists_to_disable" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.modifier_lists_to_enable {
            r = r.json(json!({ "modifier_lists_to_enable" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn modifier_lists_to_disable(
        mut self,
        modifier_lists_to_disable: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .modifier_lists_to_disable = Some(
            modifier_lists_to_disable
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn modifier_lists_to_enable(
        mut self,
        modifier_lists_to_enable: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .modifier_lists_to_enable = Some(
            modifier_lists_to_enable.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateItemModifierListsRequest<'a> {
    type Output = httpclient::InMemoryResult<UpdateItemModifierListsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}