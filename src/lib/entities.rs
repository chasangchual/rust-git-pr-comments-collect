use chrono::DateTime;

struct Repository {
    _pid: i32;
    owner: String,
    repository: String,
    created_at: DateTime,
    updated_at: DateTime,
}

struct PullRequest {
    _pid: i32;
    repository_pid: i32;
    number: i32;
    endpoint: String,
    title: String,
    body: String,
    created_at: DateTime,
    updated_at: DateTime,
}

struct Comments {
    _pid: i32;
    pr_pid: i32;
    number: i32;
    endpoint: String,
    body: String,
    diff_hunk: String,
    path: String,
    html_url: String,
    created_at: DateTime,
    updated_at: DateTime,
}
