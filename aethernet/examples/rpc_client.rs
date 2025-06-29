// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

mod interfaces;

use crate::interfaces::calculator;

#[tokio::main]
async fn main() {
    println!("Starting RPC client");

    let connection_string = std::env::args()
        .nth(1)
        .unwrap_or("redis://127.0.0.1/".into());

    // you can create an instance which just holds an open redis connection
    let client = calculator::CalculatorClient::new(&connection_string).await;

    // the client can do both gets and calls. Do a get here for fun
    match client.get_set_once().await {
        Ok(message) => println!("GET message: {message}"),
        Err(e) => println!("GET message FAILED: {e:?}"),
    }

    let my_numbers = vec![10, 20, 30];
    match client.vec_fn(&my_numbers).await {
        Ok(result) => println!("CALL:vec_fn: {my_numbers:?} -> {result:?}"),
        Err(err) => println!("Failed to call vec_fn: {err:?}"),
    }

    let mut count = 0u32;
    let my_message = "This is my message".to_string();
    loop {
        let (a, b) = (rand::random_range(0..=10), rand::random_range(0..=10));

        match client.add(a, b).await {
            Ok(sum) => println!("CALL:sub: {a} - {b} = {sum}"),
            Err(e) => println!("CALL:add: FAILED {e:?}"),
        }

        let _ = client.string_test(count, &my_message).await;
        count += 1;

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}
