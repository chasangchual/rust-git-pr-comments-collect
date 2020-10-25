use chrono::{DateTime, TimeZone, Utc};

pub struct Repository {
    _pid: i32,
    owner: String,
    repository: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

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


impl Repository {
    pub fn new(pid: i32, owner: String, repository: String) -> Repository {
        Repository {
            _pid: pid,
            owner: owner,
            repository: repository,
            created_at: None,
            updated_at: None,
        }
    }

    // Immutable access.
    pub fn pid(&self) -> &i32 {
        &self._pid
    }
    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn repository(&self) -> &String {
        &self.repository
    }
}
