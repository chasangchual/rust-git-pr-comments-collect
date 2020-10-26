
/*
use lib::entities::{Repository};
use lib::pull_request::collect_pull_request;
use lib::db::*;
*/
#![macro_use] 
extern crate diesel;
extern crate git_pr_collect;

use git_pr_collect::*;
use git_pr_collect::db::models::*;
use diesel::prelude::*;
use git_pr_collect::db::connection::establish_connection;
use git_pr_collect::github::pull_request::collect_pull_request;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_collect().await;
}

async fn run_collect() {
    use git_pr_collect::db::schema::git_repository::dsl::*;

    let connection: PgConnection = establish_connection();

    let results = git_repository.load::<GitRepository>(&connection).expect("Error load git repository");
    
    for result in results {
        println!("owner: {}, repository: {}", result.owner, result.repository);
        collect_pull_request(&result.owner, &result.repository).await.unwrap()
    }
}