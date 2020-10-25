#[cfg(feature = "postgres")]
table! {
    repository (pid) {
        pid -> Integer,
        owner -> Text,
        repository -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}