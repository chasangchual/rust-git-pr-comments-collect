use diesel::pg::data_types::PgTimestamp;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::dsl::{max};
use super::connection::PgPooledConnection;
use super::schema;
use super::schema::git_repository;
use super::schema::git_repository::dsl::*;
use super::schema::pull_request;
use super::schema::pull_request::dsl::*;
use super::schema::comments;
use super::schema::comments::dsl::*;

use diesel::sql_types::BigInt;
use chrono::{DateTime, Duration, FixedOffset, Utc};

#[derive(Queryable)]
pub struct GitRepository {
    pub pid: i32,
    pub owner: String,
    pub repository: String,
    pub number: i64,
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

#[derive(Insertable)]
#[table_name="git_repository"]
pub struct NewGitRepository {
    pub owner: String,
    pub repository: String,
    pub number: i64,
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

    pub fn exists(connection: &PgPooledConnection, _repository_num: i64) -> bool {
        let results = git_repository
                            .filter(git_repository::number.eq(_repository_num))
                            .load::<GitRepository>(connection)
                            .expect("Error loading git repository");
        results.len() >= 1
    }
    
    pub fn recent_repository_num(connection: &PgPooledConnection) -> i64 {
        let result: Option<i64> = git_repository
                            .select(max(git_repository::number))
                            .first(connection)
                            .expect("Error loaing git repository");
        match result {
            Some(value) => value,
            None => 1,
        }
    }

    pub fn create(connection: &PgPooledConnection, _repository_number: i64, _owner: String, _repository: String, 
                                                   _full_name: String, _private: bool, _description: String, 
                                                   _language: String, _url: String, _size: i32, _stargazers_count: i32, 
                                                   _watchers_count: i32, _forks_count: i32, _open_issues_count: i32, 
                                                   _open_issues: i32, _watchers: i32,  _subscribers_count: i32,
                                                   _created_at: String, _updated_at:String)  -> Result<GitRepository, Error> {

        let repository_created_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(_created_at.as_str()) {
                Ok(date) => date.with_timezone(&Utc),
                Err(_) => Utc::now(),
        };
        
        let repository_updated_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(_updated_at.as_str()){
            Ok(date) => date.with_timezone(&Utc),
            Err(_) => Utc::now(),
        };
        
        let new_git_repository = NewGitRepository {
            owner: _owner,
            repository: _repository,
            number: _repository_number,
            full_name: _full_name,
            private: _private,
            description: _description,
            language: _language,
            url: _url,
            size: _size,
            stargazers_count: _stargazers_count,
            watchers_count: _watchers_count,
            forks_count: _forks_count,
            open_issues_count: _open_issues_count,
            open_issues: _open_issues,
            watchers: _watchers,
            subscribers_count: _subscribers_count,
            created_at: PgTimestamp(repository_created_at.timestamp_millis()),
            updated_at: PgTimestamp(repository_updated_at.timestamp_millis()),
        };

        let response = diesel::insert_into(git_repository::table)
                .values(&new_git_repository)
                .get_result::<GitRepository>(connection);

        response
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

        let response = diesel::insert_into(pull_request::table)
                .values(&new_pull_request)
                .get_result::<PullRequest>(connection);

        response
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

        let response = diesel::insert_into(comments::table)
                .values(&new_comment)
                .get_result::<Comments>(connection);

        response
    }
}

