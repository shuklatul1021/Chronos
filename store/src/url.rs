use dotenvy::dotenv;
use std::env;

pub struct DatabaseCredentials {
    pub db_url : String 
}

impl Default for DatabaseCredentials {
    fn default() -> Self {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self { db_url }
    }
}

