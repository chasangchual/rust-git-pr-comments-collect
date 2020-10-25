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
