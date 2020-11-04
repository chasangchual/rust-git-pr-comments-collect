use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error;
use std::fmt;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub struct Error(Option<String>);

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(error::Error::description(self))?;
        if let Some(ref err) = self.0 {
            write!(fmt, ": {}", err)?;
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "timed out waiting for connection"
    }
}


fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set"); 
    init_pool(&database_url).expect("failed to create db connection pool")
}

pub fn get_connection(connection_pool: &PgPool) -> Result<PgPooledConnection, Error> {
    match connection_pool.clone().get() {
        Ok(connection) => Ok(connection),
        Err(error) => Err(Error(Some(error.to_string()))),
    }
}