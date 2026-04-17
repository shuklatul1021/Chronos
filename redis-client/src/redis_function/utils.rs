use redis::Commands;
use std::num::NonZeroUsize;

use crate::redis::RedisClient;

impl RedisClient {
    pub fn submit_redis_job(
        &mut self,
        key: &str, 
        value: &str
    ) -> Result<bool, redis::RedisError> {
        let con: Result<i32, redis::RedisError> = self.client.lpush(key, value);
        match con {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub fn get_job(&mut self, 
        key: &str, 
        count: u32
    ) -> Result<Option<String>, redis::RedisError> {
        let non_zero_count = NonZeroUsize::new(count as usize);
        let con: Result<Option<String>, redis::RedisError> = self.client.rpop(key, non_zero_count);
        match con {
            Ok(job) => Ok(job),
            Err(e) => Err(e),
        }
    }
}
