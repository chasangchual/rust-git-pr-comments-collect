
/*
use lib::entities::{Repository};
use lib::pull_request::collect_pull_request;
use lib::db::*;
*/
#![macro_use] extern crate diesel;

use git_pr_collect::github::pull_request::*;
use git_pr_collect::db::connection::establish_connection;
use git_pr_collect::db::models::*;
use git_pr_collect::db::schema::*;
use diesel::{PgConnection, ConnectionResult, ConnectionError};
use postgres::{Client, NoTls, Error};

#[tokio::main]
async fn main() {
    run_collect();
}

async fn run_collect() {
    use git_pr_collect::db::schema::git_repository::dsl::*;

    // let client = Client::connect("postgresql://postgres:Abc12345!@localhost:5678/git-pr-comments-collect",  NoTls);
    let connection: PgConnection = establish_connection();

    // println!("{:?}", connection.isConnected());

    //let result = git_repository.select("*").load(&connection).expect("Error load git repository").unwrap();

    //git_repository.select("").load(&connection).expect("Error load git repository").unwrap();
/*    
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
    */
}