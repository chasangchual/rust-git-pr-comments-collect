#![macro_use] 
use reqwest::header::LINK;
use reqwest::Error;
use reqwest::blocking::Response;
use parse_link_header::parse;
use serde_json::{from_str, Value, to_string_pretty};
use diesel::*;
use super::super::db::models::{PullRequest, NewPullRequest};
use super::super::db::connection::*;
use super::super::rest_client::base_client::*;
use regex::Regex;

pub fn collect_repositories(connection_pool: &PgPool) -> Result<(), Error> {
    let start_endpoint = format!("https://api.github.com/repositories?page=1&per_page=100");
    let mut endpoint = start_endpoint;

    let base_client = BaseClient::new();

    let mut has_next: bool = true;

    println!("get repositories- {}", endpoint);

    while has_next {
        let res = base_client.get(&endpoint);
        let next_link = match res {
            Ok(response) => {
                let next_link = get_next_link(&response);
                println!("next_link: {:?}", next_link);

                let body = response.text()?;    
                // parse_json(connection_pool, repository_id, owner, repository, &body);
                next_link
            },
            Err(_) => None,
        };

        has_next = next_link.is_some();
        endpoint = match next_link {
            Some(link) => link,
            None => String::from(""),
        };
    }

    Ok(())
}

fn get_next_link(response: &Response) -> Option<String> {
    let headers = response.headers();

    if headers.get(LINK).is_some() {
        // println!("headers.get(LINK).unwrap().to_str(): {:?}", headers.get(LINK).unwrap().to_str().unwrap());
        // <https://api.github.com/repositories?page=1&per_page=100&since=369>; rel=\"next\", <https://api.github.com/repositories{?since}>; rel=\"first\""

        let link_header = headers.get(LINK).unwrap().to_str().unwrap();
        let re = Regex::new(r"\{.*\}").unwrap();
        let link_header = re.replace_all(link_header, "");

        let link = parse(link_header.into_owned().as_str());
        match link {
            Ok(v) => {
                let next_link = v.get(& Some(String::from("next")));
                println!("next_link from the link header: {:?}", next_link);
                match next_link {
                    Some(v) => Some(v.raw_uri.clone()),
                    None => None
                }
            },
            Err(_) => None
        }
    } else {
        None
    }
}