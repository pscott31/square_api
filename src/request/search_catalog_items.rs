use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SearchCatalogItemsRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub category_ids: Option<Vec<String>>,
    pub cursor: Option<String>,
    pub custom_attribute_filters: Option<Vec<CustomAttributeFilter>>,
    pub enabled_location_ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub product_types: Option<Vec<String>>,
    pub sort_order: Option<String>,
    pub stock_levels: Option<Vec<String>>,
    pub text_filter: Option<String>,
}
impl<'a> SearchCatalogItemsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SearchCatalogItemsResponse> {
        let mut r = self.http_client.client.post("/v2/catalog/search-catalog-items");
        if let Some(ref unwrapped) = self.category_ids {
            r = r.json(json!({ "category_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_attribute_filters {
            r = r.json(json!({ "custom_attribute_filters" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.enabled_location_ids {
            r = r.json(json!({ "enabled_location_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.json(json!({ "limit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.product_types {
            r = r.json(json!({ "product_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sort_order {
            r = r.json(json!({ "sort_order" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stock_levels {
            r = r.json(json!({ "stock_levels" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.text_filter {
            r = r.json(json!({ "text_filter" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn category_ids(
        mut self,
        category_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .category_ids = Some(
            category_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn custom_attribute_filters(
        mut self,
        custom_attribute_filters: Vec<CustomAttributeFilter>,
    ) -> Self {
        self.custom_attribute_filters = Some(custom_attribute_filters);
        self
    }
    pub fn enabled_location_ids(
        mut self,
        enabled_location_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .enabled_location_ids = Some(
            enabled_location_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn product_types(
        mut self,
        product_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .product_types = Some(
            product_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn sort_order(mut self, sort_order: &str) -> Self {
        self.sort_order = Some(sort_order.to_owned());
        self
    }
    pub fn stock_levels(
        mut self,
        stock_levels: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .stock_levels = Some(
            stock_levels.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn text_filter(mut self, text_filter: &str) -> Self {
        self.text_filter = Some(text_filter.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for SearchCatalogItemsRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchCatalogItemsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}