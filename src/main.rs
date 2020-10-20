
mod lib;

use lib::entities::{Repository};
use lib::pull_request::collect_pull_request;
use postgres::{Client, NoTls, Error};

#[tokio::main]
async fn main() {
    run_collect().await;
}

async fn run_collect() {

    let client = Client::connect("postgresql://postgres:Abc12345!@localhost:5678/git-pr-comments-collect",  NoTls);

    match client {
        Ok(mut client) =>  {
                    let result = client.query("SELECT pid, owner, repository FROM repository", &[]);
                    match result {
                        Ok(rows) => {
                            for row in rows {
                                let repository = Repository::new(row.get(0), row.get(1), row.get(2));
                                println!("owner: {}, repository: {}", repository.owner(), repository.repository());
                                collect_pull_request(repository.owner(), repository.repository()).await.unwrap();
                            }
                        },
                        Err(e) => println!("{:?}", e),
                    }
                },
        Err(e) => println!("{:?}", e),
    }
}