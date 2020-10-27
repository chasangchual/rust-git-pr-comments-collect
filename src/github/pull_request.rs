use reqwest::Error;
use reqwest::Client;
use serde_json::{from_str, Value, to_string_pretty};
use std::env;
use diesel::*;
use super::super::db::models::{PullRequest, NewPullRequest};

pub async fn collect_pull_request(conn: &PgConnection, repository_id: i32, owner: &str, repository: &str) -> Result<(), Error> {
    let token: String = env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN must set");

    let endpoint = format!("https://api.github.com/repos/{owner}/{repository}/pulls", owner=owner, repository=repository);

    let response = Client::new()
    .get(&endpoint)
    .bearer_auth(&(token.as_str()))
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36")
    .send().await?;

    let out = response.text().await;

    match out {
        Ok(body) => parse_json(conn, repository_id, owner, repository, &body).await,
        Err(e) => println!("{:?}", e)
    };
    
    Ok(())
}
    
async fn parse_json(conn: &PgConnection, repository_id: i32, _owner: &str, _repository: &str, json_str: &str) {
    let parsed: Value = from_str(&json_str).unwrap();    
    println!("is_array: {:?}", parsed.is_array());
    println!("is_object: {:?}", parsed.is_object());

    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            // println!("{}",to_string_pretty(obj).unwrap());
            parse_pr(conn, repository_id, _owner, _repository, obj).await;
        }
    } else if parsed.is_object() {
        for node in parsed.as_object().unwrap() {
            println!("{:?}", node);
        }
    }
}

async fn parse_pr(conn: &PgConnection, repository_id: i32, _owner: &str, _repository: &str, json_root: &Value) {
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
                print_str_json("body", v);
                //body = String::from(v.as_str().unwrap());
            },
            None => ()
        }

        match json_nodes.get("_links") {
            Some(v) =>  {
                match get_chile_obj_json(v, "review_comments") {
                    Some(v) =>  {
                        // collect commit review comments
                        let href = v.as_object().unwrap().get("href").unwrap();
                        print_str_json("review_comments", href);
                        // let mut rt = tokio::runtime::Runtime::new().unwrap();
                        // rt.block_on(collect_commits(owner, repository, href.as_str().unwrap()));
                        collect_commits(_owner, _repository, href.as_str().unwrap()).await;
                    },
                    None => ()
                }
            },
            None => ()
        }

        println!("pull_request_number: {}", pull_request_number);
        println!("repository_id: {}", repository_id);

        create_pull_request(conn, &repository_id, &(pull_request_number as i32), title.as_str(), url.as_str(), body.as_str());
    }
}
pub fn create_pull_request<'a>(conn: &PgConnection, repository_id: &'a i32, pr_id: &'a i32, title: &'a str, url: &'a str, body: &'a str) -> PullRequest {
    use super::super::db::schema::pull_request;
    let new_pull_request = NewPullRequest {
        repository_id: repository_id,
        number: pr_id,
        title: title,
        endpoint: url,
        body: body,
    };

    diesel::insert_into(pull_request::table)
        .values(&new_pull_request)
        .get_result(conn)
        .expect("Error saving new pull request")
    
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
pub async fn collect_commits(_owner: &str, _repository: &str, endpoint: &str) -> Result<(), Error>   {
    let token: String = env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN must set");

    let response = Client::new()
    .get(endpoint)
    .bearer_auth(token.as_str())
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36")
    .send().await?;

    let out = response.text().await;


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