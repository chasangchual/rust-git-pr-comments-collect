use reqwest::Error;
use reqwest::Client;
use serde_json::{from_str, Value, to_string_pretty};

pub async fn collect_pull_request(owner: &str, repository: &str) -> Result<(), Error> {
    let endpoint = format!("https://api.github.com/repos/{owner}/{repository}/pulls", owner=owner, repository=repository);

    let response = Client::new()
    .get(&endpoint)
    .bearer_auth("61724104093f978d3f82d5d4221308f798739c19")
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36")
    .send().await?;

    let out = response.text().await;

    match out {
        Ok(body) => parse_json(body),
        Err(e) => println!("{:?}", e)
    };
    
    Ok(())
}
    
    
pub fn parse_json(json_str: String) {
    let parsed: Value = from_str(&json_str).unwrap();    
    println!("is_array: {:?}", parsed.is_array());
    println!("is_object: {:?}", parsed.is_object());

    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            // println!("{}",to_string_pretty(obj).unwrap());
            parse_body(obj);
        }
    } else if parsed.is_object() {
        for node in parsed.as_object().unwrap() {
            println!("{:?}", node);
        }
    }
}

pub fn parse_body(json_root: &Value) {
    if json_root.is_object() {
        let json_nodes = json_root.as_object().unwrap();

        match json_nodes.get("title") {
            Some(v) => print_str_json(v),
            None => ()
        }

        match json_nodes.get("body") {
            Some(v) => print_str_json(v),
            None => ()
        }
    }
}

pub fn print_str_json(json_node: &Value) {
    if json_node.is_string() {
        println!("{}", json_node.as_str().unwrap());
    }
}