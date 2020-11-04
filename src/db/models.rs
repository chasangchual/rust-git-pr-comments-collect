use diesel::pg::data_types::PgTimestamp;
use super::schema;
use super::schema::git_repository;
use super::schema::pull_request;
use super::connection::PgPooledConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct GitRepository {
    pub pid: i32,
    pub owner: String,
    pub repository: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Queryable)]
pub struct PullRequest {
    pub pid: i32,
    pub repository_pid: i32,
    pub number: i32,
    pub endpoint: String,
    pub title: String,
    pub body: String,
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
        use super::schema::git_repository::dsl::*;

        let results = git_repository.load::<GitRepository>(connection).expect("Error load git repository");
        results
    }
}