table! {
    comments (pid) {
        pid -> Int4,
        pr_id -> Int4,
        number -> Int4,
        endpoint -> Varchar,
        body -> Text,
        diff_hunk -> Text,
        path -> Varchar,
        html_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    git_repository (pid) {
        pid -> Int4,
        owner -> Varchar,
        repository -> Varchar,
        number -> Int4,
        full_name -> Varchar,
        private -> Bool,
        description -> Text,
        language -> Varchar,
        url -> Varchar,
        size -> Int4,
        stargazers_count -> Int4,
        watchers_count -> Int4,
        forks_count -> Int4,
        open_issues_count -> Int4,
        open_issues -> Int4,
        watchers -> Int4,
        subscribers_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    pull_request (pid) {
        pid -> Int4,
        repository_id -> Int4,
        number -> Int4,
        title -> Varchar,
        body -> Text,
        endpoint -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(comments -> pull_request (pr_id));
joinable!(pull_request -> git_repository (repository_id));

allow_tables_to_appear_in_same_query!(
    comments,
    git_repository,
    pull_request,
);
