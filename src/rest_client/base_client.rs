use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Error, StatusCode};
use reqwest::blocking::{Client, Response};
use std::env;
use url::{Url, ParseError};
use std::{thread, time};
use serde_json::from_str;

#[derive(Clone)]
pub struct BaseClient {
    client: Client,
    api_token: String,
    headers: HeaderMap,
}

// Rate limt
// https://developer.github.com/apps/building-github-apps/understanding-rate-limits-for-github-apps/#:~:text=All%20OAuth%20applications%20authorized%20by,per%20hour%20for%20that%20user.
// https://developer.github.com/v3/#rate-limiting
// {
//    "message": "API rate limit exceeded for xxx.xxx.xxx.xxx. (But here's the good news: Authenticated requests get a higher rate limit. Check out the documentation for more details.)",
//    "documentation_url": "https://developer.github.com/v3/#rate-limiting"
// }

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
        /*
        // url parse test        
        println!("endpoint to get: {}", endpoint);

        let url = Url::parse(endpoint);
        
        match url {
            Ok(url) => println!("parsed url: {}", url),
            Err(error) => (),
        }
        */
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

    pub fn get_retry_in_hit_limit(&self, endpoint: &str, count: i32) -> Result<Response, Error> {
        println!("count: {}", count);

        if count > 6 {
            panic!("it is 6th calling. stop get_retry_in_hit_limit")
        }

        let http_result: Result<Response, Error> = self.get(endpoint);
        
        match http_result {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => Ok(response),
                    StatusCode::FORBIDDEN => {
                        let remaining: i32 = self.get_ratelimit_remaining(&response);
                        if remaining > 0 {
                            Ok(response)        
                        } else {
                            println!("it reached the max hourly request rate, will sleep 10 mins ");
                            thread::sleep(time::Duration::from_millis(10 * 60 * 1000));
                            println!("make the next call with endpoint: {} - {} th", endpoint, count + 1);
                            self.get_retry_in_hit_limit(endpoint, count + 1)
                        }
                    },
                    _ => Ok(response),
                }
            },
            Err(error) => Err(error),
        }
    }    

    pub fn get_ratelimit_remaining(&self, response: &Response) -> i32 {
        // X-RateLimit-Remaining: 
        response.headers().get("X-RateLimit-Remaining")
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(5000)
    } 

    pub fn get_as_json(&self, endpoint: &str) -> Result<serde_json::value::Value, Error> {
        let headers = self.headers.clone();

        match self.client
        .get(endpoint)
        .bearer_auth(&(self.api_token.as_str()))
        .headers(headers)
        .send() 
        .and_then(|mut r| r.json()) {
            Ok(response) => Ok(response),
            Err(error) =>  {
                println!("{}", error);
                Err(error)
            },
        }    
    }    
}