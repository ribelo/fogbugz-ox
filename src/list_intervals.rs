use chrono::NaiveDateTime;
use derivative::Derivative;
use serde::Serialize;
use thiserror::Error;

use crate::{FogbugzApi, ResponseError};

#[derive(Debug, Serialize)]
pub struct ListIntervalsRequest {
    #[serde(rename = "ixBug", skip_serializing_if = "Option::is_none")]
    case_id: Option<u64>,
    #[serde(rename = "ixPerson", skip_serializing_if = "Option::is_none")]
    person: Option<u64>,
    #[serde(rename = "dtStart", skip_serializing_if = "Option::is_none")]
    start_date: Option<NaiveDateTime>,
    #[serde(rename = "dtEnd", skip_serializing_if = "Option::is_none")]
    end_date: Option<NaiveDateTime>,
    token: String,
    #[serde(skip)]
    api: FogbugzApi,
}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct ListIntervalsRequestBuilder {
    case_id: Option<u64>,
    #[derivative(Default(value = "Some(1)"))]
    person: Option<u64>,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    api: Option<FogbugzApi>,
}

#[derive(Debug, Error)]
pub enum ListIntervalsRequestBuilderError {
    #[error("Api is not specified")]
    ApiNotSpecified,
}

impl ListIntervalsRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn case_id(mut self, case_id: u64) -> Self {
        self.case_id = Some(case_id);
        self
    }
    pub fn person(mut self, person: u64) -> Self {
        self.person = Some(person);
        self
    }
    pub fn start_date(mut self, start_date: NaiveDateTime) -> Self {
        self.start_date = Some(start_date);
        self
    }
    pub fn end_date(mut self, end_date: NaiveDateTime) -> Self {
        self.end_date = Some(end_date);
        self
    }
    pub fn api(mut self, api: FogbugzApi) -> Self {
        self.api = Some(api);
        self
    }
    pub fn build(self) -> Result<ListIntervalsRequest, ListIntervalsRequestBuilderError> {
        let api = self
            .api
            .ok_or(ListIntervalsRequestBuilderError::ApiNotSpecified)?;
        let token = api.api_key.clone();
        Ok(ListIntervalsRequest {
            case_id: self.case_id,
            person: self.person,
            start_date: self.start_date,
            end_date: self.end_date,
            token,
            api,
        })
    }
}

impl ListIntervalsRequest {
    pub async fn send(self) -> Result<serde_json::Value, ResponseError> {
        let url = format!("{}/{}", self.api.url, "api/listIntervals");
        let json = serde_json::to_value(&self)?;
        dbg!(&json);
        let response = self.api.client.post(&url).json(&self).send().await?;
        dbg!(&response);
        let response = response.json::<serde_json::Value>().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FogbugzApiBuilder;

    #[tokio::test]
    async fn test_list_intervals_request() {
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

        let start_date =
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("2024-01-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();

        let request = api
            .list_intervals()
            // .person(75)
            .start_date(start_date)
            .end_date(end_date)
            .build()
            .unwrap();

        let res = request.send().await;
        dbg!(res);
    }
}
