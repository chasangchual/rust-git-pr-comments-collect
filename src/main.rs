
#![macro_use] 
extern crate diesel;
extern crate git_pr_collect;

use git_pr_collect::db::models::*;
use git_pr_collect::db::connection::*;
use git_pr_collect::github::pull_request::collect_pull_request;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    run_collect();
}

fn run_collect() {
    let pg_pool = establish_connection();

    match get_connection(&pg_pool) {
        Ok(connection) => {
                let results = GitRepository::list_all(&connection);                
                for result in results {
                    println!("owner: {}, repository: {}", result.owner, result.repository);
                    collect_pull_request(&pg_pool, result.pid, &result.owner, &result.repository).unwrap()
                }
            },
        Err(error) => println!("{}", error),                
    }
}