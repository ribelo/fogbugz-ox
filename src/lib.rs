pub mod case_details;
pub mod columns;
pub mod date;
pub mod export;
pub mod list_cases;
pub mod list_intervals;
pub mod query;
pub mod search;

use core::fmt;
use std::sync::Arc;

#[cfg(feature = "leaky-bucket")]
use leaky_bucket::RateLimiter;
use thiserror::Error;

#[derive(Clone)]
pub struct FogbugzApi {
    pub url: String,
    pub api_key: String,
    #[cfg(feature = "leaky-bucket")]
    limiter: Arc<RateLimiter>,
    pub client: reqwest::Client,
}

impl fmt::Debug for FogbugzApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FogbugzApi")
            .field("url", &self.url)
            .field("api_key", &"********")
            .finish()
    }
}

#[derive(Default)]
pub struct FogbugzApiBuilder {
    url: Option<String>,
    api_key: Option<String>,
    #[cfg(feature = "leaky-bucket")]
    limiter: Option<RateLimiter>,
    pub client: Option<reqwest::Client>,
}

#[derive(Debug, Error)]
pub enum FogbugzApiBuilderError {
    #[error("Url is not specified")]
    MissingUrl,
    #[error("Api key is not specified")]
    MissingApiKey,
    #[cfg(feature = "leaky-bucket")]
    #[error("Limiter is not specified")]
    MissingLimiter,
}

impl FogbugzApi {
    pub fn builder() -> FogbugzApiBuilder {
        FogbugzApiBuilder::default()
    }
}

impl FogbugzApiBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn url(mut self, url: impl AsRef<str>) -> Self {
        self.url = Some(url.as_ref().to_string());
        self
    }
    pub fn api_key(mut self, api_key: impl AsRef<str>) -> Self {
        self.api_key = Some(api_key.as_ref().to_string());
        self
    }
    pub fn client(mut self, client: &reqwest::Client) -> Self {
        self.client = Some(client.clone());
        self
    }
    #[cfg(feature = "leaky-bucket")]
    pub fn limiter(mut self, limiter: leaky_bucket::RateLimiter) -> Self {
        self.limiter = Some(limiter);
        self
    }
    pub fn build(self) -> Result<FogbugzApi, FogbugzApiBuilderError> {
        let url = self.url.ok_or(FogbugzApiBuilderError::MissingUrl)?;
        let api_key = self.api_key.ok_or(FogbugzApiBuilderError::MissingApiKey)?;
        #[cfg(feature = "leaky-bucket")]
        let limiter = self.limiter.ok_or(FogbugzApiBuilderError::MissingLimiter)?;
        let client = self.client.unwrap_or_default();
        Ok(FogbugzApi {
            url,
            api_key,
            #[cfg(feature = "leaky-bucket")]
            limiter: Arc::new(limiter),
            client,
        })
    }
}

impl FogbugzApi {
    pub fn list_cases(&self) -> list_cases::ListCasesRequestBuilder {
        list_cases::ListCasesRequestBuilder::new().api(self.clone())
    }
    pub fn case_details(&self) -> case_details::CaseDetailsRequestBuilder {
        case_details::CaseDetailsRequestBuilder::new().api(self.clone())
    }
    pub fn search(&self) -> search::SearchRequestBuilder {
        search::SearchRequestBuilder::new().api(self.clone())
    }
    pub fn list_intervals(&self) -> list_intervals::ListIntervalsRequestBuilder {
        list_intervals::ListIntervalsRequestBuilder::new().api(self.clone())
    }
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
    #[error("FogBugz error: {0}")]
    FogbugzError(serde_json::Value),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}
