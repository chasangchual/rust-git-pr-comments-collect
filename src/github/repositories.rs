#![macro_use] 
use reqwest::header::LINK;
use reqwest::Error;
use reqwest::blocking::Response;
use parse_link_header::parse;
use serde_json::{from_str, Value, to_string_pretty};
use diesel::*;
use super::super::db::models::{GitRepository};
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
                process_repositories(connection_pool, &body);
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

fn process_repositories(onnection_pool: &PgPool, json_str: &str) {
    // println!("{}",json_str);

    let parsed: Value = from_str(&json_str).unwrap();    
    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            // println!("{}",to_string_pretty(obj).unwrap());
            process_repository(onnection_pool, obj);
        }
    }
}

fn process_repository(onnection_pool: &PgPool, json_obj: &Value) {
    if json_obj.is_object() {
        let json_nodes = json_obj.as_object().unwrap();

        let mut id: i64 = -1;
        let mut url: String = String::from("") ;

        match json_nodes.get("id") {
            Some(v) => {
                id = v.as_i64().unwrap();
            },
            None => ()
        }

        match json_nodes.get("url") {
            Some(v) => {
                url = String::from(v.as_str().unwrap());
            },
            None => ()
        }

        process_repository_detail(onnection_pool, url.as_str());
    }
}

fn process_repository_detail(connection_pool: &PgPool, repository_url: &str) {
    let base_client = BaseClient::new();
    let res = base_client.get(repository_url);

    match res {
        Ok(response) => {
            let body = response.text().unwrap(); 
            let json_obj: Value = from_str(&body).unwrap();  
            if json_obj.is_object() {
                let json_nodes = json_obj.as_object().unwrap();
        
                let mut id: i64 = -1;
                let mut name: String = String::from("") ;
                let mut full_name: String = String::from("") ;
                let mut private: bool = false;
                let mut description: String = String::from("") ;
                let mut url: String = String::from("") ;
                let mut owner: String = String::from("") ;
        
                let mut size: i64 = -1;
                let mut stargazers_count: i64 = -1;
                let mut watchers_count: i64 = -1;
                let mut language: String = String::from("") ;
                let mut forks_count: i64 = -1;
                let mut open_issues_count: i64 = -1;
                let mut open_issues: i64 = -1;
                let mut watchers: i64 = -1;
                let mut subscribers_count: i64 = -1;


                match json_nodes.get("id") {
                    Some(v) => {
                        id = v.as_i64().unwrap();
                    },
                    None => ()
                }
                match json_nodes.get("name") {
                    Some(v) => {
                        name = String::from(v.as_str().unwrap());
                    },
                    None => ()
                }
                match json_nodes.get("full_name") {
                    Some(v) => {
                        full_name = String::from(v.as_str().unwrap());
                    },
                    None => ()
                }
                match json_nodes.get("private") {
                    Some(v) => {
                        private = v.as_bool().unwrap();
                    },
                    None => ()
                }
                match json_nodes.get("description") {
                    Some(v) => {
                        match  v.as_str() {
                            Some(v) => {
                                description = String::from(v);
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
                match json_nodes.get("url") {
                    Some(v) => {
                        url = String::from(v.as_str().unwrap());
                    },
                    None => ()
                }
                match json_nodes.get("owner") {
                    Some(v) =>  {
                        owner = v.as_object().unwrap().get("login").unwrap().as_str().unwrap().to_string();
                    },
                    None => ()
                }
        
                match json_nodes.get("subscribers_count") {
                    Some(v) => {
                        subscribers_count = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("watchers") {
                    Some(v) => {
                        watchers = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("open_issues") {
                    Some(v) => {
                        open_issues = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("open_issues_count") {
                    Some(v) => {
                        open_issues_count = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("forks_count") {
                    Some(v) => {
                        forks_count = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("language") {
                    Some(v) => {
                        match  v.as_str() {
                            Some(v) => {
                                language = String::from(v);
                            },
                            None => ()
                        }
                    },
                    None => ()
                }

                match json_nodes.get("watchers_count") {
                    Some(v) => {
                        watchers_count = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("stargazers_count") {
                    Some(v) => {
                        stargazers_count = v.as_i64().unwrap();
                    },
                    None => ()
                }

                match json_nodes.get("size") {
                    Some(v) => {
                        size = v.as_i64().unwrap();
                    },
                    None => ()
                }

                println!("{} {} {} {} {} {} {}", id, full_name, private, description, url, owner, language);
                match get_connection(&connection_pool) {
                    Ok(connection) => {
                        if ! GitRepository::exists(&connection, id) {
                            GitRepository::create(&connection, id, owner, name, full_name, private, description,
                            language, url, size as i32, stargazers_count as i32, watchers_count as i32, forks_count as i32,
                        open_issues_count as i32, open_issues_count as i32, watchers_count as i32, subscribers_count as i32 );
                        }        
                    },
                    Err(error) => println!("get_connection error {:?}", error),                
                };    
}
        },
        Err(error) => println!("{:?}", error), 
    };
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