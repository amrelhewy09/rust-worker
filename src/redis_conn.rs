use redis::Commands;
use redis::Connection;
use redis::FromRedisValue;
pub struct RedisClient {
    pub conn: Connection,
}

impl RedisClient {
    pub fn set_key(&mut self, key: String, value: u32) {
      let _ : () = self.conn.set(key, value).unwrap();
    }

    pub fn get(&mut self, key: String)-> u32 {
      let value = self.conn.get::<String, u32>(key).unwrap();

      value
    }

    pub fn brpop(&mut self) -> Option<String>{
      let value:Option<(String, String)> = self.conn.brpop("jobs", 3).unwrap();
      return match value {
        Some(value) => Some(value.1.into()),
        None => None
      }
    }
}
