use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub struct DBCredentials {
    db_url : String 
}


impl Default for DBCredentials {
    fn default() -> Self {
        let db_url = 
    }
}
