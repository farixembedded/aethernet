// NOTE: these integration tests require a Redis server running on localhost:6379

//! Test all types available to Aethernet

use aethernet::AethernetHandlerGuard;
use lazy_static::lazy_static;
use redis::AsyncTypedCommands;
use rstest::{fixture, rstest};
use std::ops::{Deref, DerefMut};
use tokio::sync::Mutex;

const CON: &str = "redis://127.0.0.1/";

#[aethernet::interface]
mod test_interface {
    trait Rpc {
        /// Add 10 to input
        fn rpc_u8(arg: u8) -> u8;
        /// Add 10 to input
        fn rpc_i8(arg: i8) -> i8;
        /// Add 10 to input
        fn rpc_u16(arg: u16) -> u16;
        /// Add 10 to input
        fn rpc_i16(arg: i16) -> i16;
        /// Add 10 to input
        fn rpc_u32(arg: u32) -> u32;
        /// Add 10 to input
        fn rpc_i32(arg: i32) -> i32;
        /// Add 10 to input
        fn rpc_u64(arg: u64) -> u64;
        /// Add 10 to input
        fn rpc_i64(arg: i64) -> i64;
        /// Add 10 to input
        fn rpc_f32(arg: f32) -> f32;
        /// Add 10 to input
        fn rpc_f64(arg: f64) -> f64;
        /// negate the bool argument
        fn rpc_bool(arg: bool) -> bool;
        /// Append "world" to the string
        fn rpc_string(arg: String) -> String;
        /// Add 10 to u8 if Result it T
        /// Append "world" to String if Result is E
        fn rpc_result(arg: Result<u8, String>) -> Result<u8, String>;
        /// return arg + 10 if Some, None if None
        fn rpc_option(arg: Option<u8>) -> Option<u8>;
        // return a vec with each element having 10 added
        fn rpc_vec(arg: Vec<u8>) -> Vec<u8>;
        /// add 10 to each value in the array
        fn rpc_array(arg: [i32; 4]) -> [i32; 4];
        /// add 10 to the u8, append "world" to the string
        fn rpc_tuple(arg: (u8, String)) -> (u8, String);
    }
    trait Pubsub {
        fn pubsub_u8(arg: u8);
        fn pubsub_i8(arg: i8);
        fn pubsub_u16(arg: u16);
        fn pubsub_i16(arg: i16);
        fn pubsub_u32(arg: u32);
        fn pubsub_i32(arg: i32);
        fn pubsub_u64(arg: u64);
        fn pubsub_i64(arg: i64);
        fn pubsub_f32(arg: f32);
        fn pubsub_f64(arg: f64);
        fn pubsub_bool(arg: bool);
    }
}

struct RpcHandler {}
impl TestInterfaceRpcHandlers for RpcHandler {
    async fn rpc_u8(&self, arg: u8) -> u8 {
        arg + 10
    }

    async fn rpc_i8(&self, arg: i8) -> i8 {
        arg + 10
    }

    async fn rpc_u16(&self, arg: u16) -> u16 {
        arg + 10
    }

    async fn rpc_i16(&self, arg: i16) -> i16 {
        arg + 10
    }

    async fn rpc_u32(&self, arg: u32) -> u32 {
        arg + 10
    }

    async fn rpc_i32(&self, arg: i32) -> i32 {
        arg + 10
    }

    async fn rpc_u64(&self, arg: u64) -> u64 {
        arg + 10
    }

    async fn rpc_i64(&self, arg: i64) -> i64 {
        arg + 10
    }

    async fn rpc_f32(&self, arg: f32) -> f32 {
        arg + 10.0
    }

    async fn rpc_f64(&self, arg: f64) -> f64 {
        arg + 10.0
    }

    async fn rpc_bool(&self, arg: bool) -> bool {
        !arg
    }

    async fn rpc_string(&self, arg: String) -> String {
        arg + "world"
    }

    async fn rpc_result(&self, arg: Result<u8, String>) -> Result<u8, String> {
        match arg {
            Ok(arg) => Ok(arg + 10),
            Err(arg) => Err(arg + "world"),
        }
    }

    async fn rpc_option(&self, arg: Option<u8>) -> Option<u8> {
        arg.map(|arg| arg + 10)
    }

    async fn rpc_vec(&self, arg: Vec<u8>) -> Vec<u8> {
        arg.iter().map(|arg| arg + 10).collect()
    }

    async fn rpc_array(&self, arg: [i32; 4]) -> [i32; 4] {
        arg.map(|x| x + 10)
    }

    async fn rpc_tuple(&self, arg: (u8, String)) -> (u8, String) {
        (arg.0 + 10, arg.1 + "world")
    }
}

// used so only one test can use redis at a time
lazy_static! {
    static ref REDIS_MUTEX: Mutex<()> = Mutex::new(());
}

struct ClientTestContext<'a> {
    client: TestInterfaceClient,
    _server_guard: AethernetHandlerGuard,
    _mutex: tokio::sync::MutexGuard<'a, ()>,
}

impl<'a> Deref for ClientTestContext<'a> {
    type Target = TestInterfaceClient;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<'a> DerefMut for ClientTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

/// Fixture to get a lock on redis, start an Aethernet server, and return the client (with mutext
/// and sever guards hidden in the struct)
#[fixture]
async fn client<'a>() -> ClientTestContext<'a> {
    // todo: take the mutex
    let mutex = REDIS_MUTEX.lock().await;

    // clear redis before test
    let client = redis::Client::open(CON).unwrap();
    let mut con = client.get_multiplexed_tokio_connection().await.unwrap();
    con.flushall().await.unwrap();

    let server_guard = TestInterfaceRpcServer::spawn_handler(CON, RpcHandler {}.into()).await;
    let client = TestInterfaceClient::new(CON).await;

    ClientTestContext {
        client,
        _server_guard: server_guard,
        _mutex: mutex,
    }
}

fn float_equal<T: num_traits::Float>(a: T, b: T) -> bool {
    (a - b).abs() < T::from(1e-10).unwrap()
}

macro_rules! integer_rpc_test {
    ($ty:ty) => {
        paste::paste! {
            #[rstest]
            #[tokio::test]
            #[awt]
            async fn [< test_rpc_ $ty >]<'a>(#[future] client: ClientTestContext<'a>) {
                assert_eq!(25, client.[<rpc_ $ty>](15).await.unwrap());
            }
        }
    };
}

integer_rpc_test!(u8);
integer_rpc_test!(i8);
integer_rpc_test!(u16);
integer_rpc_test!(i16);
integer_rpc_test!(u32);
integer_rpc_test!(i32);
integer_rpc_test!(u64);
integer_rpc_test!(i64);

macro_rules! float_rpc_test {
    ($ty:ty) => {
        paste::paste! {
            #[rstest]
            #[tokio::test]
            #[awt]
            async fn [< test_rpc_ $ty >]<'a>(#[future] client: ClientTestContext<'a>) {
                assert!(float_equal(25., client.[<rpc_ $ty>](15.).await.unwrap()));
            }
        }
    };
}

float_rpc_test!(f32);
float_rpc_test!(f64);

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_bool<'a>(#[future] client: ClientTestContext<'a>) {
    assert!(client.rpc_bool(false).await.unwrap());
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_string<'a>(#[future] client: ClientTestContext<'a>) {
    // test that it can take &str as the call arg
    assert_eq!(
        String::from("hello world"),
        client.rpc_string("hello ").await.unwrap()
    );
}

#[rstest]
#[case(Ok(5), Ok(15))]
#[case(Err("hello ".into()), Err("hello world".into()))]
#[tokio::test]
#[awt]
async fn test_rpc_result<'a>(
    #[case] arg: Result<u8, String>,
    #[case] expected: Result<u8, String>,
    #[future] client: ClientTestContext<'a>,
) {
    assert_eq!(expected, client.rpc_result(arg).await.unwrap());
}

#[rstest]
#[case(Some(5), Some(15))]
#[case(None, None)]
#[tokio::test]
#[awt]
async fn test_rpc_option<'a>(
    #[case] arg: Option<u8>,
    #[case] expected: Option<u8>,
    #[future] client: ClientTestContext<'a>,
) {
    assert_eq!(expected, client.rpc_option(arg).await.unwrap());
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_vec<'a>(#[future] client: ClientTestContext<'a>) {
    assert_eq!(
        vec![11u8, 20, 35, 60],
        client.rpc_vec(&[1, 10, 25, 50]).await.unwrap()
    );
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_array<'a>(#[future] client: ClientTestContext<'a>) {
    assert_eq!(
        [11, 20, 35, 60],
        client.rpc_array(&[1, 10, 25, 50]).await.unwrap()
    );
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_tuple<'a>(#[future] client: ClientTestContext<'a>) {
    assert_eq!(
        (11, String::from("hello world")),
        client.rpc_tuple(&(1, "hello ".into())).await.unwrap()
    );
}
