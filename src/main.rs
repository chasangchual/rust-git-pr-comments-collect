
#![macro_use] 
extern crate diesel;
extern crate git_pr_collect;

use git_pr_collect::db::models::*;
use git_pr_collect::db::connection::*;
use git_pr_collect::github::pull_request::collect_pull_request;
use git_pr_collect::github::comments::collect_review_comment;
use git_pr_collect::github::repositories::collect_repositories;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    run_collect_repositories();
    //run_pull_request_collect();
    //run_review_comment_collect();
}

fn run_collect_repositories() {
    let pg_pool = establish_connection();

    match get_connection(&pg_pool) {
        Ok(connection) => {
            collect_repositories(&pg_pool).unwrap()
        },
        Err(error) => println!("{}", error),                
    }
}

fn run_pull_request_collect() {
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

fn run_review_comment_collect() {
    let pg_pool = establish_connection();

    match get_connection(&pg_pool) {
        Ok(connection) => {
                let results = PullRequest::list_all(&connection);                
                for result in results {
                    println!("pid: {}, repository_id: {}, pr number: {}, title: {}, endpoint: {}", 
                            result.pid, result.repository_pid, result.number, result.title, result.endpoint);
                            collect_review_comment(&pg_pool, &result);
                }
            },
        Err(error) => println!("{}", error),                
    }
}