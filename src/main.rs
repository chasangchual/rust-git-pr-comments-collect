
mod lib;

use lib::entities::{Repository};
use lib::pull_request::collect_pull_request;
use postgres::{Client, NoTls, Error};

#[tokio::main]
async fn main() {

    let owner = "apache";
    let repository = "kafka";
    match run_collect() {
        Ok(r) => r,
        Err(e) => println!("{:?}", e),
    }

    collect_pull_request(owner, repository).await.unwrap();
}


fn run_collect() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/repository",  NoTls)?;

    for row in client.query("SELECT pid, owner, courepositoryntry FROM author", &[])? {
        let repository = Repository::new(row.get(0), row.get(1), row.get(2));
        println!("Author {} is from {}", repository.owner(), repository.repository());
    }

    Ok(())
}