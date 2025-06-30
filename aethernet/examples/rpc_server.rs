// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

// Show usage of the rpc interface in Aethernet
mod interfaces;

use crate::interfaces::calculator;
use tokio::sync::Mutex;

// ths struct handles incoming RPC calls
struct RpcHandler {
    // data must be synchronized to be shared across handlers
    rpc_counter: Mutex<u32>,
}

impl RpcHandler {
    // you can share state among the different handlers
    async fn increment_counter(&self) {
        let mut rpc_counter = self.rpc_counter.lock().await;
        *rpc_counter += 1;
        println!("RPC CALL COUNT: {rpc_counter}");
    }
}

// impl [SERVICE]RpcHandlers trait to create the RPC handlers
impl calculator::CalculatorRpcHandlers for RpcHandler {
    async fn add(&self, a: i32, b: i32) -> i32 {
        self.increment_counter().await;
        let sum = a + b;
        println!("RPC:add: {a} + {b} = {sum}");
        sum
    }

    async fn sub(&self, a: i32, b: i32) -> i32 {
        self.increment_counter().await;
        let diff = a - b;
        println!("RPC:sub: {a} - {b} = {diff}");
        diff
    }

    async fn test(&self, test_intput: i32) {
        self.increment_counter().await;
        println!("RPC:test: test_input: {test_intput}");
    }

    async fn string_test(&self, count: u32, message: String) {
        self.increment_counter().await;
        println!("RPC:string_test: {count}:{message}");
    }

    async fn vec_fn(&self, input: Vec<i32>) -> Vec<i32> {
        input.iter().map(|val| val + 1).collect()
    }
}

#[tokio::main]
async fn main() {
    println!("Starting RPC server");

    let connection_string = std::env::args()
        .nth(1)
        .unwrap_or("redis://127.0.0.1/".into());

    // spawn a handler to service incoming RPC calls in the background. A scoped guard is returned
    // to keep track of the underlying RPC handler task. When it drops the task will be aborted and
    // the now further RPC calls will be handled.
    let _calc_guard = calculator::CalculatorRpcServer::spawn_handler(
        &connection_string,
        RpcHandler {
            rpc_counter: 0.into(),
        }
        .into(),
    );

    // wait forever and server RPC
    futures::future::pending::<()>().await;
}
