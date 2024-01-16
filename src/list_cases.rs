use reqwest::Url;
use serde::Serialize;
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

#[derive(Debug, Default)]
pub struct ListCasesRequestBuilder {
    filter: Option<String>,
    cols: Option<Vec<String>>,
    max: Option<u32>,
    api: Option<FogbugzApi>,
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
    pub fn cols(mut self, cols: Vec<Column>) -> Self {
        self.cols = Some(cols.into_iter().map(|s| s.to_string()).collect());
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

impl ListCasesRequest {
    pub fn builder() -> ListCasesRequestBuilder {
        ListCasesRequestBuilder::new()
    }
    pub async fn send(&self) -> Result<(), ResponseError> {
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
            let data: serde_json::Value = response.json().await?;
            dbg!(data);
            Ok(())
        } else {
            let data: serde_json::Value = response.json().await?;
            dbg!(data);
            Ok(())
            // Err(ResponseError::MistralResponseError(data))
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
            .add_col(Column::Title)
            .add_col(Column::CaseId)
            .build()
            .unwrap();
        request.send().await.unwrap();
    }
}
