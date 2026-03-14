use redis::Client;

pub fn init_redis() -> Client {
    let redis_url = std::env::var("REDIS_URL").unwrap();
    Client::open(redis_url).expect("Failed to connect to redis server")
}
