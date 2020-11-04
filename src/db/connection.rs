use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub struct DBClient {
    connection: PgConnection,
}

impl DBClient {
    pub fn new() -> DBClient {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
        DBClient {
            connection: PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url)),
        }
    }
}
