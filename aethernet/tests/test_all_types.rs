// NOTE: these integration tests require a Redis server running on localhost:6379

//! Test all types available to Aethernet

mod common;

use aethernet::AethernetHandlerGuard;
use common::valkey_con_str;
use lazy_static::lazy_static;
use redis::AsyncTypedCommands;
use rstest::{fixture, rstest};
use tokio::sync::Mutex;

#[aethernet::interface]
mod type_test_interface {
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
        /// append "world" to string, or None if input is None
        fn rpc_option(arg: Option<String>) -> Option<String>;
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
        fn pubsub_string(arg: String);
        fn pubsub_result(arg: Result<u8, String>);
        fn pubsub_option(arg: Option<String>);
        fn pubsub_vec(arg: Vec<u8>);
        fn pubsub_array(arg: [u8; 4]);
        fn pubsub_tuple(arg: (u8, String));
    }
}

struct RpcHandler {}
impl TypeTestInterfaceRpcHandlers for RpcHandler {
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

    async fn rpc_option(&self, arg: Option<String>) -> Option<String> {
        arg.map(|arg| arg + "world")
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

struct TestIpcContext<'a> {
    client: TypeTestInterfaceClient,
    publish: TypeTestInterfacePublisher,
    _server_guard: AethernetHandlerGuard,
    _mutex: tokio::sync::MutexGuard<'a, ()>,
}

/// Fixture to get a lock on redis, start an Aethernet server along with client and publisher
#[fixture]
async fn ipc<'a>() -> TestIpcContext<'a> {
    // todo: take the mutex
    let mutex = REDIS_MUTEX.lock().await;

    // clear redis before test
    let client = redis::Client::open(valkey_con_str()).unwrap();
    let mut con = client.get_multiplexed_tokio_connection().await.unwrap();
    con.flushall().await.unwrap();

    let server_guard =
        TypeTestInterfaceRpcServer::spawn_handler(&valkey_con_str(), RpcHandler {}.into()).await;
    let client = TypeTestInterfaceClient::new(&valkey_con_str()).await;
    let publish = TypeTestInterfacePublisher::new(&valkey_con_str()).await;

    TestIpcContext {
        client,
        publish,
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
            async fn [< test_rpc_ $ty >]<'a>(#[future] ipc: TestIpcContext<'a>) {
                assert_eq!(25, ipc.client.[<rpc_ $ty>](15).await.unwrap());
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
            async fn [< test_rpc_ $ty >]<'a>(#[future] ipc: TestIpcContext<'a>) {
                assert!(float_equal(25., ipc.client.[<rpc_ $ty>](15.).await.unwrap()));
            }
        }
    };
}

float_rpc_test!(f32);
float_rpc_test!(f64);

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_bool<'a>(#[future] ipc: TestIpcContext<'a>) {
    assert!(ipc.client.rpc_bool(false).await.unwrap());
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_string<'a>(#[future] ipc: TestIpcContext<'a>) {
    // test that it can take &str as the call arg
    assert_eq!(
        String::from("hello world"),
        ipc.client.rpc_string("hello ").await.unwrap()
    );
}

#[rstest]
#[case::ok(Ok(5), Ok(15))]
#[case::err(Err("hello ".into()), Err("hello world".into()))]
#[tokio::test]
#[awt]
async fn test_rpc_result<'a>(
    #[case] arg: Result<u8, String>,
    #[case] expected: Result<u8, String>,
    #[future] ipc: TestIpcContext<'a>,
) {
    assert_eq!(expected, ipc.client.rpc_result(arg).await.unwrap());
}

#[rstest]
#[case::some(Some("hello ".into()), Some("hello world".into()))]
#[case::none(None, None)]
#[tokio::test]
#[awt]
async fn test_rpc_option<'a>(
    #[case] arg: Option<String>,
    #[case] expected: Option<String>,
    #[future] ipc: TestIpcContext<'a>,
) {
    assert_eq!(expected, ipc.client.rpc_option(arg).await.unwrap());
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_vec<'a>(#[future] ipc: TestIpcContext<'a>) {
    assert_eq!(
        vec![11u8, 20, 35, 60],
        ipc.client.rpc_vec(&[1, 10, 25, 50]).await.unwrap()
    );
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_array<'a>(#[future] ipc: TestIpcContext<'a>) {
    assert_eq!(
        [11, 20, 35, 60],
        ipc.client.rpc_array(&[1, 10, 25, 50]).await.unwrap()
    );
}

#[rstest]
#[tokio::test]
#[awt]
async fn test_rpc_tuple<'a>(#[future] ipc: TestIpcContext<'a>) {
    assert_eq!(
        (11, String::from("hello world")),
        ipc.client.rpc_tuple(&(1, "hello ".into())).await.unwrap()
    );
}

macro_rules! pubsub_test {
    // 2 args: no description, so use "base"
    ($ty:ident, $send:expr) => {
        pubsub_test!(@inner $ty, $send, $send, base);
    };

    // 3 args: if 3rd arg is an ident, treat as suffix/desc
    ($ty:ident, $send:expr, $desc:ident) => {
        pubsub_test!(@inner $ty, $send, $send, $desc);
    };

    // 3 args: if 3rd arg is an expr, treat as recv value, desc = base
    ($ty:ident, $send:expr, $recv:expr) => {
        pubsub_test!(@inner $ty, $send, $recv, base);
    };

    // 4 args: full explicit
    ($ty:ident, $send:expr, $recv:expr, $desc:ident) => {
        pubsub_test!(@inner $ty, $send, $recv, $desc);
    };

    // Internal arm: generate the test function
    (@inner $ty:ident, $send:expr, $recv:expr, $desc:ident) => {
        paste::paste! {
            #[rstest]
            #[tokio::test]
            #[awt]
            async fn [< test_pubsub_ $ty _ $desc >]<'a>(#[future] ipc: TestIpcContext<'a>) {
                ipc.publish.[<pubsub_ $ty>]($send).await.unwrap();
                assert_eq!($recv, ipc.client.[<get_pubsub_ $ty>]().await.unwrap());
            }
        }
    };
}

pubsub_test!(u8, 5);
pubsub_test!(i8, 5);
pubsub_test!(i16, 5);
pubsub_test!(u16, 5);
pubsub_test!(i32, 5);
pubsub_test!(u32, 5);
pubsub_test!(i64, 5);
pubsub_test!(u64, 5);
pubsub_test!(f32, 5.);
pubsub_test!(f64, 5.);
pubsub_test!(bool, true);
pubsub_test!(string, "hello world", String::from("hello world"));
pubsub_test!(result, Ok(5), case_ok);
pubsub_test!(result, Err("hello world".into()), case_err);
pubsub_test!(option, Some("hello world".into()), case_some);
pubsub_test!(option, None, case_none);
pubsub_test!(vec, &[1u8, 2, 3], vec![1u8, 2, 3]);
pubsub_test!(array, &[1u8, 2, 3, 4], [1u8, 2, 3, 4]);
pubsub_test!(tuple, &(5u8, "hello".into()), (5u8, "hello".into()));
