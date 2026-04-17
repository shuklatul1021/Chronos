use diesel::prelude::*;
use redis_client::redis::RedisClient;

use crate::url::DatabaseCredentials;

pub struct Store {
    pub connection: PgConnection,
    pub redis_client: RedisClient,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = DatabaseCredentials::default();
        let connection = PgConnection::establish(&config.db_url);
        let redis_client = RedisClient::new();
        match (connection, redis_client) {
            (Ok(conn), Ok(redis)) => Ok(Self {
                connection: conn,
                redis_client: redis,
            }),
            (Ok(_), Err(redis_error)) => {
                let redis_error_message = format!("Bad Redis Connection {redis_error}");
                Err(ConnectionError::BadConnection(redis_error_message))
            }
            (Err(pg_error), Ok(_)) => {
                let pg_error_message = format!("Bad Database Connection {pg_error}");
                Err(ConnectionError::BadConnection(pg_error_message))
            }
            (Err(pg_error), Err(redis_error)) => {
                let error_message = format!(
                    "Bad Database Connection {pg_error}; Bad Redis Connection {redis_error}"
                );
                Err(ConnectionError::BadConnection(error_message))
            }
        }
    }
}
