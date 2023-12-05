use redis::AsyncCommands;
use rocket::tokio;
// Function to connect to an external Redis database
pub async fn connect_to_redis(redis_url: &str) -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open(redis_url)?;
    let connection = client.get_async_connection().await?;
    Ok(connection)
}

// Test for the connect_to_redis function
#[tokio::test]
async fn test_connect_to_redis() {
    // Replace "redis://your-redis-server:6379" with your actual Redis server details
    let redis_url = "rediss://red-ckt7ltg168ec738dodcg:xIEJbECEzPI6xaVZWYMHi90OSS2jliN2@oregon-redis.render.com:6379";

    // Attempt to connect to Redis
    match connect_to_redis(redis_url).await {
        Ok(mut connection) => {
            // If the connection is successful, perform some basic operations (e.g., set and get)
            let _: () = connection.set("test_key", "test_value").await.unwrap();
            let result: Option<String> = connection.get("test_key").await.unwrap();
            assert_eq!(result, Some("test_value".to_string()));
        }
        Err(err) => {
            // If there is an error, fail the test
            panic!("Failed to connect to Redis: {:?}", err);
        }
    }
}
