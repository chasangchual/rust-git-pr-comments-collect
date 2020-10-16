
mod lib;

use lib::pull_request::collect_pull_request;

#[tokio::main]
async fn main() {
    let owner = "apache";
    let repository = "kafka";

    collect_pull_request(owner, repository).await.unwrap();
}
