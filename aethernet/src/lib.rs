// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

// warnings are errors in release builds. Use `debug_assertions` as a surrogate to check for debug builds
#![cfg_attr(not(debug_assertions), deny(warnings))]
// don't allow unwrapping
#![warn(clippy::unwrap_used)]

// ********** Public API surface
pub use rpc_client::AethernetRpcClient;
pub use server::AethernetServer;
pub use subscriber::AethernetSubscriber;
pub use types::{AethernetError, AethernetHandlerGuard, AethernetPubsub, AethernetRpc};

// Re-export proc macro for generating Aethernet interfaces
pub use aethernet_macros::interface;

// re-export deps for use within the proc macro
#[doc(hidden)]
pub mod _deps {
    pub use async_trait;
    pub use serde;
    pub use serde_json;
    pub use trait_variant;
}

// ********** Crate/module private stuff
mod keys;
mod rpc_client;
mod server;
mod subscriber;
mod types;
