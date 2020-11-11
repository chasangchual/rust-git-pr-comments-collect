
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

pub fn collect_pull_request(connection_pool: &PgPool, repository_id: i32, owner: &str, repository: &str) -> Result<(), Error> {
    let start_endpoint = format!("https://api.github.com/repos/{owner}/{repository}/pulls?page=1&per_page=100", owner=owner, repository=repository);
    let mut endpoint = start_endpoint;

    let base_client = BaseClient::new();

    let mut has_next: bool = true;

    while has_next {
        let res = base_client.get(&endpoint);
        let next_link = match res {
            Ok(response) => {
                let next_link = get_next_link(&response);
                println!("next_link: {:?}", next_link);

                let body = response.text()?;    
                parse_json(connection_pool, repository_id, owner, repository, &body);
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
        let link = parse(headers.get(LINK).unwrap().to_str().unwrap());
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
    
fn parse_json(onnection_pool: &PgPool, repository_id: i32, _owner: &str, _repository: &str, json_str: &str) {
    let parsed: Value = from_str(&json_str).unwrap();    
    println!("is_array: {:?}", parsed.is_array());
    println!("is_object: {:?}", parsed.is_object());

    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            // println!("{}",to_string_pretty(obj).unwrap());
            parse_pr(onnection_pool, repository_id, _owner, _repository, obj);
        }
    } else if parsed.is_object() {
        for node in parsed.as_object().unwrap() {
            println!("{:?}", node);
        }
    }
}

fn parse_pr(connection_pool: &PgPool, repository_id: i32, _owner: &str, _repository: &str, json_root: &Value) {
println!("json_root is_array: {:?}", json_root.is_array());
println!("json_root is_object: {:?}", json_root.is_object());

    if json_root.is_object() {
        let json_nodes = json_root.as_object().unwrap();
        let mut pull_request_number: i64 = -1;
        let mut title: String = String::from("") ;
        let mut body: String = String::from("") ;
        let mut url: String = String::from("") ;

        match json_nodes.get("number") {
            Some(v) => {
                print_str_json("number", v);
                pull_request_number = v.as_i64().unwrap();
            },
            None => ()
        }

        match json_nodes.get("url") {
            Some(v) =>  {
                print_str_json("url", v);
                url = String::from(v.as_str().unwrap());
            },
            None => ()
        }

        match json_nodes.get("title") {
            Some(v) =>  {
                print_str_json("title", v);
                title = String::from(v.as_str().unwrap());
            },
            None => ()
        }

        match json_nodes.get("body") {
            Some(v) =>  {
                // print_str_json("body", v);
                body = match v.as_str() {
                    Some(b) => String::from(b),
                    None => String::from(""),
                }
            },
            None => ()
        }

        println!("pull_request_number: {}", pull_request_number);
        println!("repository_id: {}", repository_id);
 
        match get_connection(&connection_pool) {
            Ok(connection) => {
                if ! PullRequest::exists(&connection, repository_id, pull_request_number as i32) {
                    PullRequest::create(&connection, repository_id, pull_request_number as i32, title, body, url);
                }        
            },
            Err(error) => println!("{:?}", error),                
        };        

        match json_nodes.get("_links") {
            Some(v) =>  {
                match get_chile_obj_json(v, "review_comments") {
                    Some(v) =>  {
                        // collect commit review comments
                        let href = v.as_object().unwrap().get("href").unwrap();
                        print_str_json("review_comments", href);
                        // let mut rt = tokio::runtime::Runtime::new().unwrap();
                        // rt.block_on(collect_commits(owner, repository, href.as_str().unwrap()));
                        
                        // collect_commits(_owner, _repository, href.as_str().unwrap());
                    },
                    None => ()
                }
            },
            None => ()
        }
    }
}

fn print_str_json(name: &str, json_node: &Value) {
    if json_node.is_string() {
        println!("{name}: {value}", name = name, value = json_node.as_str().unwrap());
    } else if  json_node.is_number() {
        println!("{name}: {value}", name = name, value = json_node.as_i64().unwrap());
    } else {
        println!("{name} is not a string node", name = name);
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


// https://api.github.com/repos/apache/kafka/pulls/9418/comments
pub fn collect_commits(_owner: &str, _repository: &str, endpoint: &str) -> Result<(), Error>   {
    let base_client = BaseClient::new();

    let response = base_client.get(endpoint);

    let out = response.unwrap().text();

    match out {
        Ok(body) => {
            let parsed: Value = from_str(&body).unwrap();    
            if parsed.is_array() {
                for obj in parsed.as_array().unwrap() {
                    // println!("{}",to_string_pretty(obj).unwrap());
                    if obj.is_object() {
                        let json_nodes = obj.as_object().unwrap();
                
                        match json_nodes.get("id") {
                            Some(v) => print_str_json("id", v),
                            None => ()
                        }
                
                        match json_nodes.get("url") {
                            Some(v) => print_str_json("url", v),
                            None => ()
                        }
                
                        match json_nodes.get("body") {
                            Some(v) => print_str_json("body", v),
                            None => ()
                        }

                        match json_nodes.get("diff_hunk") {
                            Some(v) => print_str_json("diff_hunk", v),
                            None => ()
                        }

                        match json_nodes.get("path") {
                            Some(v) => print_str_json("path", v),
                            None => ()
                        }

                        match json_nodes.get("_links") {
                            Some(v) =>  {
                                match get_chile_obj_json(v, "html") {
                                    Some(v) =>  {
                                        // collect commit review comments
                                        let href = v.as_object().unwrap().get("href").unwrap();
                                        print_str_json("html", href);
                                    },
                                    None => ()
                                }
                            },
                            None => ()
                        }
                    }
                                }
            } else if parsed.is_object() {
                for node in parsed.as_object().unwrap() {
                    println!("{:?}", node);
                }
            }

        },
        Err(e) => println!("{:?}", e)
    };
    
    Ok(())
} 