use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub struct DBClient {
    client: PgConnection,
    url: String,
}

impl DBClient {
    pub fn new() -> DBClient {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");

        DBClient {
            url: database_url.clone(),
            client: PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
        }
    }
}
