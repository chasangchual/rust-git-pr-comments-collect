use reqwest::Error;
use reqwest::Client;
use serde_json::from_str;
use serde_json::Value;

pub async fn collect_pull_request(owner: &str, repository: &str) -> Result<(), Error> {
    let endpoint = format!("https://api.github.com/repos/{owner}/{repository}/pulls", owner=owner, repository=repository);

    let response = Client::new()
    .get(&endpoint)
    .bearer_auth("52fbd36b149fcdd2fcacc7f00f73bda4c340349c")
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
            println!("{}", obj);
            println!("\n\n");
        }
    } else if parsed.is_object() {
        for node in parsed.as_object().unwrap() {
            println!("{:?}", node);
        }
    }
}
