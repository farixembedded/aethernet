// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::keys::AethernetKeys;
use crate::types::{AethernetPubsub, AethernetRpcRepEnvelope, AethernetRpcReqEnvelope};
use crate::{AethernetError, AethernetRpc};
use redis::AsyncTypedCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Can publish Pubsub updates as well as receive and reply to RPC ReqReps.
#[derive(Clone)]
pub struct AethernetServer {
    valkey_client: redis::Client,
    valkey_connection: Arc<Mutex<Option<redis::aio::MultiplexedConnection>>>,
    keys: AethernetKeys,
}

impl AethernetServer {
    pub async fn new(
        connection_string: &str,
        service_name: &str,
        interface_name: &str,
    ) -> Result<Self, AethernetError> {
        let valkey_client = redis::Client::open(connection_string)?;

        Ok(Self {
            valkey_client,
            valkey_connection: Arc::new(Mutex::new(None)),
            keys: AethernetKeys::new(service_name, interface_name),
        })
    }

    // TODO: pass through deserialization and redis errors
    pub async fn wait_for_and_deserialize_next_request(
        &self,
    ) -> Result<AethernetRpcReqEnvelope<serde_json::Value>, AethernetError> {
        let mut valkey = self.ensure_and_get_valkey_connetion().await?;
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
    ) -> Result<(), AethernetError> {
        let mut valkey = self.ensure_and_get_valkey_connetion().await?;
        let serialized_rep_evelope = serde_json::to_string(rep_envelope)?;

        let key_rpc_response = self.keys.rpc_response(req_id);
        valkey
            .lpush(&key_rpc_response, serialized_rep_evelope)
            .await?;
        // client has 60 seconds to get the response, after that its deleted to prevent leaks
        valkey.expire(&key_rpc_response, 60).await?;

        Ok(())
    }

    pub async fn publish<'a, T: AethernetPubsub<'a>>(
        &self,
        msg: &T::MsgRefType,
    ) -> Result<(), AethernetError> {
        let mut valkey = self.ensure_and_get_valkey_connetion().await?;
        let key_pubsub = self.keys.pubsub(T::MESSAGE_NAME);
        let key_pubsub_latest = self.keys.pubsub_latest(T::MESSAGE_NAME);

        let message = serde_json::to_string(msg)?;

        valkey.set(key_pubsub_latest, &message).await?;
        valkey.publish(key_pubsub, &message).await?;
        Ok(())
    }

    async fn ensure_and_get_valkey_connetion(
        &self,
    ) -> Result<redis::aio::MultiplexedConnection, AethernetError> {
        let first_connection;
        // check connection wth a ping
        let mut valkey_connection = self.valkey_connection.lock().await;
        if let Some(valkey_connection) = valkey_connection.as_mut() {
            first_connection = false;
            if valkey_connection.ping().await.is_ok() {
                return Ok(valkey_connection.clone());
            }
        } else {
            first_connection = true;
        }

        // try to reconnect once
        let mut new_valkey_connection = self
            .valkey_client
            .get_multiplexed_tokio_connection()
            .await?;
        *valkey_connection = Some(new_valkey_connection.clone());

        if first_connection {
            // clear out the full "[service_name]:*" key space to start fresh at least clear the RPC
            // request queue
            let key_rpc_request = self.keys.rpc_request();
            if let Err(err) = new_valkey_connection.del(&key_rpc_request).await {
                warn!("Failed to flush RPC queue on first connection: {key_rpc_request}, {err}");
            }
        }

        Ok(new_valkey_connection)
    }
}
