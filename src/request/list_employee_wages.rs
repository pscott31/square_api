use serde_json::json;
use crate::model::*;
use crate::SquareApiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListEmployeeWagesRequest<'a> {
    pub(crate) http_client: &'a SquareApiClient,
    pub cursor: Option<String>,
    pub employee_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> ListEmployeeWagesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListEmployeeWagesResponse> {
        let mut r = self.http_client.client.get("/v2/labor/employee-wages");
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.employee_id {
            r = r.query("employee_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn employee_id(mut self, employee_id: &str) -> Self {
        self.employee_id = Some(employee_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListEmployeeWagesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListEmployeeWagesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}