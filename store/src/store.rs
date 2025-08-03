use diesel::prelude::*;
use crate::config;

pub struct Store {
    pub conn: PgConnection,
}


impl Store {
    // constructor
    // conn
    fn default() -> Result<Self, ConnectionError> {
        let config = config::Config::default();
        let conn = PgConnection::establish(&config.db_url)
            .unwrap_or_else(|_| panic!("Failed to connect to database {}", config.db_url));
        Ok(Self { conn: conn })
    }
}