// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

mod interfaces;

use crate::interfaces::calculator;
use tokio::sync::Mutex;

// this is how we implement handlers for pubs coming in
struct SubHandler {
    /// example of sharing data between handlers. It must be synchronized.
    pub_counter: Mutex<u32>,
}

impl SubHandler {
    /// demonstrate keeping some internal state shared among subscription handlers
    async fn increment_counter(&self) {
        let mut pub_counter = self.pub_counter.lock().await;
        *pub_counter += 1;
        println!("PUB COUNT: {pub_counter}");
    }
}

// you impl the [Interface]PubsubHandlers trait on your struct to put in the code called when
// pubs come in. All RPC methods in the interface my be implmented.
impl calculator::CalculatorPubsubHandlers for SubHandler {
    async fn counter(&self, arg: u32) {
        self.increment_counter().await;
        println!("GOT PUB:counter: {arg}");
    }

    async fn heartbeat(&self, tick: u32, has_error: bool) {
        self.increment_counter().await;
        println!("GOT PUB:heartbeat: tick: {tick}, has_error: {has_error}");
    }

    async fn set_once(&self, arg: String) {
        self.increment_counter().await;
        println!("GOT PUB:set_once: {arg}");
    }

    async fn vec_pub(&self, nums: Vec<i32>) {
        self.increment_counter().await;
        println!("GOT PUB:vec_pub {nums:?}");
    }
}

#[tokio::main]
async fn main() {
    println!("Starting RPC server");

    let connection_string = std::env::args()
        .nth(1)
        .unwrap_or("redis://127.0.0.1/".into());

    // to get the latest value published without waiting for an incoming publish you can create an
    // instance of a client. client.get_*() is used to access the latest published values.
    let getter = calculator::CalculatorClient::new(&connection_string)
        .await
        .unwrap();
    match getter.get_counter().await {
        Ok(value) => println!("INSTANCE GET: counter: {value}"),
        Err(e) => println!("INSTANCE GET: failed: {e:?}"),
    }

    // create an subscriber instance to listen for and react to incoming subscriptions. A scoped
    // guard is returned to keep track of the underlying subscriber task. When it drops the task
    // will be aborted and the subscriber will stop seeing subscriptions.
    let _calc_guard = calculator::CalculatorSubscriber::spawn_handler(
        &connection_string,
        SubHandler {
            pub_counter: 0.into(),
        }
        .into(),
    );

    // wait forever and let the subscriber handle incoming pubs
    futures::future::pending::<()>().await;
}
