use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{columns::Column, FogbugzApi, ResponseError};

#[derive(Debug, Serialize)]
pub struct CaseDetailsRequest {
    #[serde(rename = "q")]
    case_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    cols: Option<Vec<String>>,
    token: String,
    #[serde(skip)]
    api: FogbugzApi,
}

#[derive(Debug)]
pub struct CaseDetailsRequestBuilder {
    case_id: Option<u64>,
    cols: Option<Vec<String>>,
    api: Option<FogbugzApi>,
}

impl Default for CaseDetailsRequestBuilder {
    fn default() -> Self {
        Self {
            case_id: None,
            cols: Some(vec![
                Column::TicketNumber.to_string(),
                Column::Title.to_string(),
                Column::Events.to_string(),
            ]),
            api: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum CaseDetailsRequestBuilderError {
    #[error("Ticket number is not specified")]
    TicketNumberNotSpecified,
    #[error("Api is not specified")]
    ApiNotSpecified,
}

#[derive(Debug, Deserialize)]
pub struct EventDetail {
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
#[serde(untagged)]
pub enum Event {
    Unmodified(EventDetail),
    Modified(Vec<EventDetail>),
}

#[derive(Debug, Deserialize)]
pub struct CaseDetails {
    #[serde(rename = "ixBug")]
    pub ticket_number: u64,
    #[serde(rename = "sTitle")]
    pub title: String,
    pub events: Vec<Event>,
}

impl CaseDetailsRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn case_id(mut self, ticket_number: u64) -> Self {
        self.case_id = Some(ticket_number);
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
    pub fn build(self) -> Result<CaseDetailsRequest, CaseDetailsRequestBuilderError> {
        let ticket_number = self
            .case_id
            .ok_or(CaseDetailsRequestBuilderError::TicketNumberNotSpecified)?;
        let api = self
            .api
            .ok_or(CaseDetailsRequestBuilderError::ApiNotSpecified)?;
        Ok(CaseDetailsRequest {
            case_id: ticket_number,
            cols: self.cols,
            token: api.api_key.clone(),
            api,
        })
    }
}

impl CaseDetailsRequest {
    pub fn builder() -> CaseDetailsRequestBuilder {
        CaseDetailsRequestBuilder::new()
    }
    pub async fn send(&self) -> Result<CaseDetails, ResponseError> {
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
            let case = json["data"]["cases"][0].take();
            let case_details: CaseDetails = serde_json::from_value(case)?;
            Ok(case_details)
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
    async fn test_case_details_request() {
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
            .case_details()
            .case_id(61331)
            .add_col(Column::Events)
            .add_col(Column::Body)
            .build()
            .unwrap();
        let res = request.send().await.unwrap();
    }
}
