use reqwest::Error;
use reqwest::Client;
use serde_json::{from_str, Value, to_string_pretty};

pub async fn collect_pull_request(owner: &str, repository: &str) -> Result<(), Error> {
    let endpoint = format!("https://api.github.com/repos/{owner}/{repository}/pulls", owner=owner, repository=repository);

    let response = Client::new()
    .get(&endpoint)
    .bearer_auth("")
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36")
    .send().await?;

    let out = response.text().await;

    match out {
        Ok(body) => parse_json(&body),
        Err(e) => println!("{:?}", e)
    };
    
    Ok(())
}
    
fn parse_json(json_str: &str) {
    let parsed: Value = from_str(&json_str).unwrap();    
    println!("is_array: {:?}", parsed.is_array());
    println!("is_object: {:?}", parsed.is_object());

    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            // println!("{}",to_string_pretty(obj).unwrap());
            parse_pr(obj);
        }
    } else if parsed.is_object() {
        for node in parsed.as_object().unwrap() {
            println!("{:?}", node);
        }
    }
}

fn parse_pr(json_root: &Value) {
    if json_root.is_object() {
        let json_nodes = json_root.as_object().unwrap();

        match json_nodes.get("title") {
            Some(v) => print_str_json("title", v),
            None => ()
        }

        match json_nodes.get("body") {
            Some(v) => print_str_json("pr body", v),
            None => ()
        }

        match json_nodes.get("_links") {
            Some(v) =>  {
                match get_chile_obj_json(v, "review_comments") {
                    Some(v) =>  {
                        let href = v.as_object().unwrap().get("href").unwrap();
                        print_str_json("review_comments", href);
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
        println!("{name} {value}", name = name, value = json_node.as_str().unwrap());
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