use diesel::prelude::*;

use crate::url::DatabaseCredentials;

pub struct Store {
    pub connection : PgConnection
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError>{
        let config = DatabaseCredentials::default();
        let connection = PgConnection::establish(&config.db_url);
        match connection {
            Ok(conn) => {
                Ok(Self{
                    connection: conn
                })
            },
            Err(err) => {
                let error_messsage = format!("Bad Connection {err}");
                Err(ConnectionError::BadConnection(error_messsage))
            }
        }
    }
}