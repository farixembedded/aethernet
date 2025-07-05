// NOTE: these integration tests require a Redis server running on localhost:6379

use redis::AsyncTypedCommands;

const CON: &str = "redis://127.0.0.1/";

#[aethernet::interface]
mod test_interface {
    trait Rpc {
        fn add(a: i32, b: i32) -> i32;
    }
    trait Pubsub {}
}

async fn clear_redis() {
    let client = redis::Client::open(CON).unwrap();
    let mut con = client.get_multiplexed_tokio_connection().await.unwrap();
    con.flushall().await.unwrap();
}

#[tokio::test]
async fn test() {
    clear_redis().await;

    struct RpcHandler {}
    impl TestInterfaceRpcHandlers for RpcHandler {
        async fn add(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }

    let _guard = TestInterfaceRpcServer::spawn_handler(CON, RpcHandler {}.into()).await;
    let client = TestInterfaceClient::new(CON).await;

    assert_eq!(12, client.add(5, 7).await.unwrap());
    assert_eq!(3, client.add(7, -4).await.unwrap());
}
