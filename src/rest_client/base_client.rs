use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Error;
use reqwest::blocking::{Client, Response};
use serde::Serialize;
use std::env;

#[derive(Clone)]
pub struct BaseClient {
    client: Client,
    api_token: String,
    user_agent: String,
}

impl BaseClient {
    pub fn new() -> BaseClient {
        let token: String = env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN must set");

        BaseClient {
            client: reqwest::blocking::Client::new(),
            api_token: token,
            user_agent: String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36"),
        }
    }

    /// Get a resource
    pub fn get(&self, endpoint: &str) -> Result<Response, Error> {
        match self.client
        .get(endpoint)
        .bearer_auth(&(self.api_token.as_str()))
        .header("User-Agent", self.user_agent.as_str())
        .send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }    
}