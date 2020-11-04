
/*
use lib::entities::{Repository};
use lib::pull_request::collect_pull_request;
use lib::db::*;
*/
#![macro_use] 
extern crate diesel;
extern crate git_pr_collect;

use git_pr_collect::db::models::*;
use diesel::prelude::*;
use git_pr_collect::db::connection::DBClient;
use git_pr_collect::github::pull_request::collect_pull_request;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    run_collect();
}

fn run_collect() {
    use git_pr_collect::db::schema::git_repository::dsl::*;
    
    let dbClient: DBClient = DBClient::new();

    let results = git_repository.load::<GitRepository>(&dbClient.getConnection()).expect("Error load git repository");
    
    for result in results {
        println!("owner: {}, repository: {}", result.owner, result.repository);
        collect_pull_request(&connection, result.pid, &result.owner, &result.repository).unwrap()
    }
}