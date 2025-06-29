// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

// Example interface for a simple contrived calculator
// the aethernet::interface macro generates a number of classes and traits for interacting with IPC
//
// CalculatorClient - a client to call RPC methods and get Pubsub latest values
// CalculatorPublisher - used on the server to publish updates to pubsub values
// CalculatorPubsubHandlers - trait for handling incoming pubsub subscriptions
// CalculatorRpcHandlers - trait for handling incoming RPC requests
// CalculatorRpcServer - collects incoming RPC requests and delegates to an RpcHandlers impl
// CalculatorSubscriber - collects incoming subscriptions and delegates to a PubsubHandlers impl
#[aethernet::interface]
mod calculator {
    // for RPC methods, put the function signature of the method
    trait Rpc {
        fn add(a: i32, b: i32) -> i32;
        fn sub(a: i32, b: i32) -> i32;
        fn test(test_intput: i32);
        // For client calls String becomes &str
        fn string_test(count: u32, message: String);
        // For client calls Vec<T> becomes a slice &[T]
        fn vec_fn(input: Vec<i32>) -> Vec<i32>;
    }

    // For pubsub, the function args represent the message payload. Pubsub signatures can not have a
    // return value.
    trait Pubsub {
        fn heartbeat(tick: u32, has_error: bool);
        fn counter(arg: u32);
        // For publishing, String becomes &str
        fn set_once(arg: String);
        // For publish functions Vec<T> becomes a slice &[T]
        fn vec_pub(nums: Vec<i32>);
        // the CalculatorPubsubHandlers trait has a default no-op impl for all subscriptions
        fn unimplemented_publish(dummy: i32);
    }
}
