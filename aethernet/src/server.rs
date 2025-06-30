// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use crate::keys::AethernetKeys;
use crate::types::{AethernetPubsub, AethernetRpcRepEnvelope, AethernetRpcReqEnvelope};
use crate::{AethernetError, AethernetRpc};
use redis::AsyncTypedCommands;

/// Can publish Pubsub updates as well as receive and reply to RPC ReqReps.
#[derive(Clone)]
pub struct AethernetServer {
    valkey: redis::aio::MultiplexedConnection,
    keys: AethernetKeys,
}

impl AethernetServer {
    pub async fn new(connection_string: &str, service_name: &str, interface_name: &str) -> Self {
        let valkey = {
            let client = redis::Client::open(connection_string).unwrap();
            client.get_multiplexed_tokio_connection().await.unwrap()
        };

        let mut server = Self {
            valkey,
            keys: AethernetKeys::new(service_name, interface_name),
        };

        // clear out the full "[service_name]:*" key space to start fresh
        // at least clear the RPC request queue
        let key_rpc_request = server.keys.rpc_request();
        server.valkey.del(key_rpc_request).await.unwrap();

        server
    }

    // TODO: pass through deserialization and redis errors
    pub async fn wait_for_and_deserialize_next_request(
        &self,
    ) -> Result<AethernetRpcReqEnvelope<serde_json::Value>, AethernetError> {
        let mut valkey = self.valkey.clone();
        let key_rpc_request = self.keys.rpc_request();

        let serialized_req_envelope = &valkey
            .brpop(key_rpc_request, 0.0)
            .await?
            .ok_or(AethernetError::RedisError(Default::default()))?[1];

        serde_json::from_str::<AethernetRpcReqEnvelope<serde_json::Value>>(serialized_req_envelope)
            .map_err(|err| AethernetError::SerdeError(err.to_string()))
    }

    pub async fn send_rpc_reply<'a, T: AethernetRpc<'a>>(
        &self,
        req_id: &str,
        rep_envelope: &AethernetRpcRepEnvelope<T::RepType>,
    ) {
        let mut valkey = self.valkey.clone();
        let serialized_rep_evelope = serde_json::to_string(rep_envelope).unwrap();

        let key_rpc_response = self.keys.rpc_response(req_id);
        valkey
            .lpush(&key_rpc_response, serialized_rep_evelope)
            .await
            .unwrap();
        // client has 60 seconds to get the response, after that its deleted to prevent leaks
        valkey.expire(&key_rpc_response, 60).await.unwrap();
    }

    pub async fn publish<'a, T: AethernetPubsub<'a>>(
        &self,
        msg: &T::MsgRefType,
    ) -> Result<(), AethernetError> {
        let mut valkey = self.valkey.clone();
        let key_pubsub = self.keys.pubsub(T::MESSAGE_NAME);
        let key_pubsub_latest = self.keys.pubsub_latest(T::MESSAGE_NAME);

        let message = serde_json::to_string(msg)?;

        valkey.set(key_pubsub_latest, &message).await.unwrap();
        valkey.publish(key_pubsub, &message).await.unwrap();
        Ok(())
    }
}
