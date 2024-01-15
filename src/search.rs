use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    columns::Column,
    query::{IntoQuery, Query},
    FogbugzApi, ResponseError,
};

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    #[serde(rename = "q")]
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cols: Option<Vec<String>>,
    token: String,
    #[serde(skip)]
    api: FogbugzApi,
}

#[derive(Debug)]
pub struct SearchRequestBuilder {
    query: Option<Query>,
    cols: Option<Vec<String>>,
    api: Option<FogbugzApi>,
}

impl Default for SearchRequestBuilder {
    fn default() -> Self {
        Self {
            query: None,
            cols: Some(vec![
                Column::TicketNumber.to_string(),
                Column::Title.to_string(),
            ]),
            api: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum SearchRequestBuilderError {
    #[error("Query is not specified")]
    QueryNotSpecified,
    #[error("Api is not specified")]
    ApiNotSpecified,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "evtDescription")]
    pub description: String,
    #[serde(rename = "ixPerson")]
    pub person_id: u64,
    #[serde(rename = "sPerson")]
    pub person: String,
    #[serde(rename = "s")]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CaseDetails {
    #[serde(rename = "ixBug")]
    pub ticket_number: u64,
    #[serde(rename = "sTitle")]
    pub title: String,
    pub events: Vec<Event>,
}

impl SearchRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn query(mut self, query: impl IntoQuery) -> Self {
        self.query = Some(query.into_query());
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
    pub fn api(mut self, api: FogbugzApi) -> Self {
        self.api = Some(api);
        self
    }
    pub fn build(self) -> Result<SearchRequest, SearchRequestBuilderError> {
        let query = self
            .query
            .ok_or(SearchRequestBuilderError::QueryNotSpecified)?;
        let api = self.api.ok_or(SearchRequestBuilderError::ApiNotSpecified)?;
        Ok(SearchRequest {
            query: query.to_string(),
            cols: self.cols,
            token: api.api_key.clone(),
            api,
        })
    }
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        SearchRequestBuilder::new()
    }
    pub async fn send(&self) -> Result<serde_json::Value, ResponseError> {
        let url = Url::parse(&self.api.url)?.join("api/search")?;
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
            // let case = json["data"]["cases"][0].take();
            // let case_details: CaseDetails = serde_json::from_value(case)?;
            Ok(json)
        } else {
            let json: serde_json::Value = response.json().await?;
            Err(ResponseError::FogbugzError(json))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{date::PointInTime, FogbugzApiBuilder};

    #[tokio::test]
    async fn test_search_request() {
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

        let query = Query::builder()
            .closed_date((PointInTime::new(1, 1, 2024), PointInTime::new(31, 12, 2024)))
            .build();
        let request = api.search().query(query).build().unwrap();
        let res = request.send().await.unwrap();
        dbg!(res);
    }
}
