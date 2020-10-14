use reqwest::Error;
use reqwest::Client;
use serde_json::from_str;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    collect_prs2().await
}

async fn collect_prs2() -> Result<(), Error> {

    let response = Client::new()
    .get("https://api.github.com/repos/apache/kafka/pulls")
    .bearer_auth("dbf116503a6ebe6c207d4c6564f0f4bb53abd687")
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36")
    .send().await?;

    let out = response.text().await;

    match out {
        Ok(v) =>parse_json(v),
        Err(e) => println!("{:?}", e)
    }

    Ok(())
}

fn parse_json(json_str: String) {
    let parsed: Value = from_str(&json_str).unwrap();    
    println!("{:?}", parsed);

}
