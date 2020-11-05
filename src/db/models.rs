use diesel::pg::data_types::PgTimestamp;
use super::connection::PgPooledConnection;
use diesel::prelude::*;

use super::schema;
use super::schema::git_repository;
use super::schema::git_repository::dsl::*;
use super::schema::pull_request;
use super::schema::pull_request::dsl::*;

#[derive(Queryable)]
pub struct GitRepository {
    pub pid: i32,
    pub owner: String,
    pub repository: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct PullRequest {
    pub pid: i32,
    pub repository_pid: i32,
    pub number: i32,
    pub title: String,
    pub body: String,
    pub endpoint: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Insertable)]
#[table_name="pull_request"]
pub struct NewPullRequest<'a> {
    pub repository_id: &'a i32,
    pub number: &'a i32,
    pub endpoint: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable)]
pub struct Comments {
    pub pid: i32,
    pub pr_pid: i32,
    pub number: i32,
    pub endpoint: String,
    pub body: String,
    pub diff_hunk: String,
    pub path: String,
    pub html_url: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

impl GitRepository {
    pub fn list_all(connection: &PgPooledConnection) -> Vec<GitRepository>{
        let results = git_repository.load::<GitRepository>(connection).expect("Error load git repository");
        results
    }
}


impl PullRequest {
    pub fn list_all(connection: &PgPooledConnection) -> Vec<PullRequest> {
        let results = pull_request.load::<PullRequest>(connection).expect("Error load git pull requests");
        results
    }

    pub fn get(connection: &PgPooledConnection, _repository_id: i32, _pull_request_number: i32) -> Vec<PullRequest>{
        let results = pull_request
                             .filter(repository_id.eq(_repository_id))
                             .filter(number.eq(_pull_request_number))
                             .load::<PullRequest>(connection)
                             .expect("Error loading pull requests");
        results
    }

    pub fn exists(connection: &PgPooledConnection, _repository_id: i32, _pull_request_number: i32) -> bool {
        let results = pull_request
                             .filter(repository_id.eq(_repository_id))
                             .filter(number.eq(_pull_request_number))
                             .load::<PullRequest>(connection)
                             .expect("Error loading pull requests");
        results.len() >= 1
    }
}