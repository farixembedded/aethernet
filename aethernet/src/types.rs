// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AethernetError {
    MethodNotImplemented,
    ValueNotFound,
    TimedOut,
    // TODO: should put the actual source error in here and add a 'source' impl in the impl 'Error'
    // below. These should also be the actual Error, not a String.
    SerdeError(String),
    RedisError(String),
}

impl std::fmt::Display for AethernetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AethernetError {}

impl From<redis::RedisError> for AethernetError {
    fn from(value: redis::RedisError) -> Self {
        AethernetError::RedisError(value.to_string())
    }
}

impl From<serde_json::Error> for AethernetError {
    fn from(value: serde_json::Error) -> Self {
        AethernetError::SerdeError(value.to_string())
    }
}

pub trait AethernetRpc<'a> {
    const METHOD_NAME: &'static str;
    type ReqType: Serialize + serde::de::DeserializeOwned;
    type ReqRefType: Serialize;
    type RepType: Serialize + serde::de::DeserializeOwned;
}

impl<'a> AethernetRpc<'a> for AethernetError {
    const METHOD_NAME: &'static str = "ERROR";
    type ReqType = ();
    type ReqRefType = ();
    type RepType = ();
}

pub trait AethernetPubsub<'a> {
    const MESSAGE_NAME: &'static str;
    type MsgType: Serialize + serde::de::DeserializeOwned;
    type MsgRefType: Serialize;
}

#[derive(Serialize, Deserialize)]
pub struct AethernetRpcReqEnvelope<T> {
    /// A globally unique message ID. UUID4 is suggested
    pub req_id: String,
    /// The RPC method name
    pub req_type: String,
    /// The request body
    pub req: T,
}

// TODO: can we roll the above and this struct into one?
#[derive(Serialize, Deserialize)]
pub struct AethernetRpcReqEnvelopeRawReq<'a> {
    pub req_id: String,
    pub req_type: String,
    #[serde(borrow)]
    pub req: &'a serde_json::value::RawValue,
}

// no extra data for rep envelope right now, so don't wrap it in a struct
pub type AethernetRpcRepEnvelope<T> = Result<T, AethernetError>;

// resource guard to drop the spawned RPC/pubsub handler when it goes out of scope
pub struct AethernetHandlerGuard {
    pub join_handle: ::tokio::task::JoinHandle<()>,
}

impl Drop for AethernetHandlerGuard {
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}
