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
    .bearer_auth("49c2770553f3811abc70c8cb9e82525838afab98")
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
//    println!("{}", json_str);

    let parsed: Value = from_str(&json_str).unwrap();    
    println!("is_array: {:?}", parsed.is_array());
    println!("is_object: {:?}", parsed.is_object());

    if parsed.is_array() {
        for obj in parsed.as_array().unwrap() {
            println!("{}", obj);
            println!("\n\n");
        }
    } else if parsed.is_object() {
        for key in parsed.as_object().unwrap().keys() {
            println!("{}", key);
        }
    }
}
