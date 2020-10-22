use chrono::{DateTime, TimeZone, Utc};
use diesel::Queryable;

#[derive(Queryable)]
pub struct Repository {
    _pid: i32,
    owner: String,
    repository: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct PullRequest {
    _pid: i32,
    repository_pid: i32,
    number: i32,
    endpoint: String,
    title: String,
    body: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct Comments {
    _pid: i32,
    pr_pid: i32,
    number: i32,
    endpoint: String,
    body: String,
    diff_hunk: String,
    path: String,
    html_url: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
