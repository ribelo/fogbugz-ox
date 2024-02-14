use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{enums::Column, FogbugzApi, ResponseError};

#[derive(Debug, Serialize)]
pub struct ListCasesRequest {
    #[serde(rename = "sFilter", skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<u32>,
    token: String,
    #[serde(skip)]
    api: FogbugzApi,
}

#[derive(Debug)]
pub struct ListCasesRequestBuilder {
    filter: Option<String>,
    cols: Option<Vec<String>>,
    max: Option<u32>,
    api: Option<FogbugzApi>,
}

impl Default for ListCasesRequestBuilder {
    fn default() -> Self {
        let builder = ListCasesRequestBuilder {
            filter: Default::default(),
            cols: Default::default(),
            max: Default::default(),
            api: Default::default(),
        };
        builder.cols(&[
            Column::CaseId,
            Column::Title,
            Column::Project,
            Column::ProjectId,
        ])
    }
}

#[derive(Debug, Error)]
pub enum ListCasesRequestBuilderError {
    #[error("Api is not specified")]
    ApiNotSpecified,
}

impl ListCasesRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn filter(mut self, filter: impl AsRef<str>) -> Self {
        self.filter = Some(filter.as_ref().to_string());
        self
    }
    pub fn cols(mut self, cols: &[Column]) -> Self {
        self.cols = Some(cols.iter().map(|s| s.to_string()).collect());
        self
    }
    pub fn add_col(mut self, col: Column) -> Self {
        if let Some(cols) = &mut self.cols {
            cols.push(col.to_string())
        } else {
            self.cols = Some(vec![col.to_string()]);
        }
        self
    }
    pub fn max(mut self, max: u32) -> Self {
        self.max = Some(max);
        self
    }
    pub fn api(mut self, api: FogbugzApi) -> Self {
        self.api = Some(api);
        self
    }
    pub fn build(self) -> Result<ListCasesRequest, ListCasesRequestBuilderError> {
        let api = self
            .api
            .ok_or(ListCasesRequestBuilderError::ApiNotSpecified)?;
        Ok(ListCasesRequest {
            filter: self.filter,
            cols: self.cols,
            max: self.max,
            token: api.api_key.clone(),
            api,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    #[serde(rename = "ixBug")]
    pub case_id: u64,
    #[serde(rename = "ixProject")]
    pub project_id: u64,
    #[serde(rename = "sProject")]
    pub project: String,
    #[serde(rename = "sTitle")]
    pub titile: String,
}

impl ListCasesRequest {
    pub fn builder() -> ListCasesRequestBuilder {
        ListCasesRequestBuilder::new()
    }
    pub async fn send(&self) -> Result<Vec<Case>, ResponseError> {
        let url = Url::parse(&self.api.url)?.join("api/listCases")?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "leaky-bucket")] {
                    self.api.limiter.acquire_one().await;
            }
        }
        let response = self
            .api
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api.api_key)
            .json(&self)
            .send()
            .await?;

        if response.status().is_success() {
            let mut json: serde_json::Value = response.json().await?;
            let cases = serde_json::from_value(json["data"]["cases"].take())?;
            Ok(cases)
        } else {
            let json: serde_json::Value = response.json().await?;
            Err(ResponseError::FogbugzError(json))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FogbugzApiBuilder;

    #[tokio::test]
    async fn test_list_cases_request() {
        let api_key = std::env::var("FOGBUGZ_API_KEY").unwrap();
        #[cfg(feature = "leaky-bucket")]
        let limiter = leaky_bucket::RateLimiter::builder()
            .initial(1)
            .interval(std::time::Duration::from_secs(1))
            .build();
        let api = FogbugzApiBuilder::new()
            .url("https://retailic.fogbugz.com")
            .api_key(api_key)
            .limiter(limiter)
            .build()
            .unwrap();
        let request = api
            .list_cases()
            .max(1)
            .cols(&[
                Column::Title,
                Column::CaseId,
                Column::Project,
                Column::ProjectId,
            ])
            .build()
            .unwrap();

        let res = request.send().await.unwrap();
        dbg!(&res);
    }
}
