use dotenvy::dotenv;


pub struct RedisClient {
    pub client : redis::Connection,
}


impl RedisClient {
    pub fn new() -> Result<Self, redis::RedisError> {
        dotenv().ok();
        let redis_client = redis::Client::open(std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into()))?;
        let client = redis_client.get_connection()?;

        Ok(RedisClient { client })
    }
}