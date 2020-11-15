use diesel::pg::data_types::PgTimestamp;
use diesel::prelude::*;
use diesel::result::Error;

use super::connection::PgPooledConnection;
use super::schema;
use super::schema::git_repository;
use super::schema::git_repository::dsl::*;
use super::schema::pull_request;
use super::schema::pull_request::dsl::*;
use super::schema::comments;
use super::schema::comments::dsl::*;

#[derive(Queryable)]
pub struct GitRepository {
    pub pid: i32,
    pub owner: String,
    pub repository: String,
    pub number: i32,
    pub full_name: String,
    pub private: bool,
    pub description: String,
    pub language: String,
    pub url: String,
    pub size: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub open_issues: i32,
    pub watchers: i32,
    pub subscribers_count: i32,
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
pub struct NewPullRequest {
    pub repository_id: i32,
    pub number: i32,
    pub title: String,
    pub body: String,
    pub endpoint: String,
}

#[derive(Queryable)]
pub struct Comments {
    pub pid: i32,
    pub pr_id: i32,
    pub number: i32,
    pub endpoint: String,
    pub body: String,
    pub diff_hunk: String,
    pub path: String,
    pub html_url: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComments {
    pub pr_id: i32,
    pub number: i32,
    pub endpoint: String,
    pub body: String,
    pub diff_hunk: String,
    pub path: String,
    pub html_url: String,
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

    pub fn get(connection: &PgPooledConnection, _repository_id: i32, _number: i32) -> Vec<PullRequest>{
        let results = pull_request
                             .filter(repository_id.eq(_repository_id))
                             .filter(pull_request::number.eq(_number))
                             .load::<PullRequest>(connection)
                             .expect("Error loading pull requests");
        results
    }

    pub fn exists(connection: &PgPooledConnection, _repository_id: i32, _number: i32) -> bool {
        let results = pull_request
                             .filter(repository_id.eq(_repository_id))
                             .filter(pull_request::number.eq(_number))
                             .load::<PullRequest>(connection)
                             .expect("Error loading pull requests");
        results.len() >= 1
    }

    pub fn create(connection: &PgPooledConnection, _repository_pid: i32, _number: i32, _title: String, _body: String, _endpoint: String)  -> Result<PullRequest, Error> {
        let new_pull_request = NewPullRequest {
            repository_id: _repository_pid,
            number: _number,
            title: _title,
            body: _body,
            endpoint: _endpoint,
        };

        let pull_requst = diesel::insert_into(pull_request::table)
                .values(&new_pull_request)
                .get_result::<PullRequest>(connection);

        pull_requst
    }
}

impl Comments {
    pub fn list_all(connection: &PgPooledConnection) -> Vec<Comments> {
        let results = comments.load::<Comments>(connection).expect("Error load git pull request review comments");
        results
    }

    pub fn exists(connection: &PgPooledConnection, _pr_pid: i32, _number: i32) -> bool {
        let results = comments
                             .filter(pr_id.eq(_pr_pid))
                             .filter(comments::number.eq(_number))
                             .load::<Comments>(connection)
                             .expect("Error loading review comments");
        results.len() >= 1
    }

    pub fn create(connection: &PgPooledConnection, _pr_pid: i32, _number: i32, _endpoint: String, _body: String, _diff_hunk: String, _path: String, _html_url: String)  -> Result<Comments, Error> {
        let new_comment = NewComments {
            pr_id: _pr_pid,
            number: _number,
            endpoint: _endpoint,
            body: _body,
            diff_hunk: _diff_hunk,
            path: _path,
            html_url: _html_url,
        };

        let comment = diesel::insert_into(comments::table)
                .values(&new_comment)
                .get_result::<Comments>(connection);

        comment
    }
}

