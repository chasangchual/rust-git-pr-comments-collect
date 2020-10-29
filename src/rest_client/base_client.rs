use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::Error;
use reqwest::blocking::{Client, Response};
use serde::Serialize;
use std::env;

#[derive(Clone)]
pub struct BaseClient {
    client: Client,
    api_token: String,
    headers: HeaderMap,
}

impl BaseClient {
    pub fn new() -> BaseClient {
        let token: String = env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN must set");
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36"));
        
        BaseClient {
            client: reqwest::blocking::Client::new(),
            api_token: token,
            headers: headers,
        }
    }

    /// Get a resource
    pub fn get(&self, endpoint: &str) -> Result<Response, Error> {
        let headers = self.headers.clone();

        match self.client
        .get(endpoint)
        .bearer_auth(&(self.api_token.as_str()))
        .headers(headers)
        .send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }    
}