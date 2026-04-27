use redis::Connection;

pub fn redis(redis_url: String) -> Connection {
    redis::Client::open(redis_url)
        .expect("Failed to connect to Redis")
        .get_connection()
        .unwrap()
}
