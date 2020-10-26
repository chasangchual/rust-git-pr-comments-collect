use diesel::pg::data_types::PgTimestamp;

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
