use reqwest::Error;
use serde_json::{from_str, Value, to_string_pretty};
use super::super::db::models::{PullRequest, Comments};
use super::super::db::connection::*;
use super::super::rest_client::base_client::*;

pub fn collect_review_comment(connection_pool: &PgPool, pull_request: &PullRequest) -> Result<(), Error> {
    println!("pull request url: {:?}", pull_request.endpoint);
    
    let comment_endpoint = get_review_comments_links(pull_request.endpoint.clone().as_str());

    match comment_endpoint {
        Some(endpoint) => {
            println!("get_review_comments_links: {}", endpoint.as_str());

            let base_client = BaseClient::new();
            let res = base_client.get(endpoint.as_str());

            match res {
                Ok(response) => {
                    
                    let out = response.text().unwrap();
                    // println!("comments: {}",to_string_pretty(out.as_str()).unwrap());     

                    let comments: Value = from_str(&out).unwrap();  
                    
                    if comments.is_array() {
                        for comment in comments.as_array().unwrap() {
                            let mut number: i32 = -1;
                            let mut endpoint: String = String::from("") ;
                            let mut body: String = String::from("") ;
                            let mut diff_hunk: String = String::from("") ;
                            let mut path: String = String::from("") ;
                            let mut html_url: String = String::from("") ;

                            if comment.is_object() {
                                let json_nodes = comment.as_object().unwrap();
                                
                                match json_nodes.get("id") {
                                    Some(v) => number = v.as_i64().unwrap() as i32,
                                    None => ()
                                }
                        
                                match json_nodes.get("url") {
                                    Some(v) => endpoint = String::from(v.as_str().unwrap()),
                                    None => ()
                                }
                        
                                match json_nodes.get("body") {
                                    Some(v) => body = String::from(v.as_str().unwrap()),
                                    None => ()
                                }
        
                                match json_nodes.get("diff_hunk") {
                                    Some(v) => diff_hunk = String::from(v.as_str().unwrap()),
                                    None => ()
                                }
        
                                match json_nodes.get("path") {
                                    Some(v) => path = String::from(v.as_str().unwrap()),
                                    None => ()
                                }
        
                                match json_nodes.get("_links") {
                                    Some(v) =>  {
                                        match get_chile_obj_json(v, "html") {
                                            Some(v) =>  {
                                                // collect commit review comments
                                                html_url = v.as_object().unwrap().get("href").unwrap().as_str().unwrap().to_string();
                                            },
                                            None => ()
                                        }
                                    },
                                    None => ()
                                }
                            } else {

                            }

                            println!("cr comment number: {}", number);
                            println!("cr comment endpoint: {}", endpoint);
                            println!("cr comment body: {}", body);
                            println!("cr comment diff_hunk: {}", diff_hunk);
                            println!("cr comment path: {}", path);
                            println!("cr comment html_url: {}", html_url);
                            match get_connection(&connection_pool) {
                                Ok(connection) => {
                                    if ! Comments::exists(&connection, pull_request.pid, number) {
                                        Comments::create(&connection, pull_request.pid, number, endpoint, body, diff_hunk, path, html_url);
                                    }        
                                },
                                Err(error) => println!("get_connection error {:?}", error),                
                            };    
                        }
                    } else {
                        if comments.is_object() {
                            println!("comments: {}",to_string_pretty(comments.as_str().unwrap()).unwrap());
                        } 
                    }
                },
                Err(error) => println!("{:?}", error), 
            };
        },
        None => ()
    }
    
    Ok(())
}

fn get_review_comments_links(endpoint: &str) -> Option<String> {
    let base_client = BaseClient::new();
    let response = base_client.get(endpoint);
    let out = response.unwrap().text();
    let mut found: bool = false;
    let mut href: String = String::from("");

    match out {
        Ok(body) => {
            let parsed: Value = from_str(&body).unwrap();

            match parsed.get("_links") {
                Some(v) =>  {
                    // println!("{}",to_string_pretty(v).unwrap());
                    match get_chile_obj_json(v, "review_comments") {
                        Some(v) =>  {
                            // println!("{}",to_string_pretty(v).unwrap());
                            found = true;
                            href = String::from(v.as_object().unwrap().get("href").unwrap().as_str().unwrap());
                        },
                        None => ()
                    }
                },
                None => ()
            }
        },
        Err(_) => (),
    };

    if found {
        Some(href)    
    } else {
        None
    }
}

fn get_chile_obj_json<'a>(json_node: &'a Value, child_name: &'a str) -> Option<&'a Value>{
    if json_node.is_object() {
        match json_node.as_object().unwrap().get(child_name) {
            Some(v) =>  {
                Some(v)
            },
            None => None
        }
    } else {
        None
    }
}