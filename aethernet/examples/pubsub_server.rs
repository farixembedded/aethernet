// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

mod interfaces;

use crate::interfaces::calculator;

#[tokio::main]
async fn main() {
    println!("Starting Pubsub server");

    let connection_string = std::env::args()
        .nth(1)
        .unwrap_or("redis://127.0.0.1/".into());

    // to publish you create a publisher instance and use the methods to publish new values.
    let publish = calculator::CalculatorPublisher::new(&connection_string).await;

    // do a one-off publish
    let set_once_message = format!(
        "A string set once. Rand number {}",
        rand::random_range(0..=10)
    );
    match publish.set_once(&set_once_message).await {
        Ok(_) => println!("INSTANCE PUB: set_once: {set_once_message}"),
        Err(err) => println!("Failed to publish set_one: {err:?}"),
    }

    let my_numbers = vec![1, 2, 3, 4, 10, 20];
    if let Err(err) = publish.vec_pub(&my_numbers).await {
        println!("Failed to publish vec: {err:?}");
    }

    let mut counter = 0;
    loop {
        if let Err(err) = publish.counter(counter).await {
            println!("Failed to publish counter: {err:?}");
        }
        counter += 1;
        println!("INSTANCE PUB:counter: {counter}");

        // every third second publish the heartbeat
        if counter % 3 == 0 {
            if let Err(err) = publish.heartbeat(counter, rand::random_bool(0.3)).await {
                println!("Failed to publish heartbeat: {err:?}");
            }
            println!("INSTANCE PUB:heartbeat");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
