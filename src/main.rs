mod job;
mod redis_conn;
mod thread_pool;
mod worker;
use crate::redis_conn::RedisClient;
use crate::thread_pool::ThreadPool;
use dotenv::dotenv;

const POOL_SIZE: usize = 4;
fn main() {
    dotenv().ok();
    let redis_init = redis::Client::open(std::env::var("REDIS_HOST").unwrap()).unwrap();
    let conn = redis_init.get_connection().unwrap();
    let mut redis_client = RedisClient { conn: conn };
    let pool = ThreadPool::new(POOL_SIZE);

    loop {
      match redis_client.brpop(){
        Some(val) => {
          pool.execute(move || {
            println!("Found a job in the queue, processing {}", val)
          })
        },
        None => println!("Waiting for items..")
      }

    }


}
