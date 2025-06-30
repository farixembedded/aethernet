// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::keys::AethernetKeys;
use crate::types::{AethernetError, AethernetRpcReqEnvelope};
use crate::{AethernetPubsub, AethernetRpc};

use redis::AsyncTypedCommands;
use uuid::Uuid;

/// Can send RPC requests and get the latest published value for Pubsub topics
#[derive(Clone)]
pub struct AethernetRpcClient {
    valkey: redis::aio::MultiplexedConnection,
    keys: AethernetKeys,
}

impl AethernetRpcClient {
    const DEFAULT_TIMEOUT_SECONDS: f64 = 1.0;

    pub async fn new(connection_string: &str, service_name: &str, interface_name: &str) -> Self {
        info!("Creating new client for service: {service_name}, interface: {interface_name}");

        // the Client needs resp3 protocol to support concurrent interface use and PushInfo updates
        let connection_string = match connection_string.contains("protocol=resp3") {
            true => connection_string.to_string(),
            false => match connection_string.contains("?") {
                true => format!("{connection_string}&protocol=resp3"),
                false => format!("{connection_string}?protocol=resp3"),
            },
        };
        info!("Connecting to Redis at: {connection_string}");

        let valkey = {
            let client = redis::Client::open(connection_string).unwrap();
            client.get_multiplexed_tokio_connection().await.unwrap()
        };

        let keys = AethernetKeys::new(service_name, interface_name);
        info!(
            "Client initialized. RPC request queue: {}",
            keys.rpc_request()
        );

        Self { valkey, keys }
    }

    pub async fn call<'a, T: AethernetRpc<'a>>(
        &self,
        req: T::ReqRefType,
    ) -> Result<T::RepType, AethernetError> {
        let mut valkey = self.valkey.clone();

        let req_id = Uuid::new_v4().to_string();
        let method_name = T::METHOD_NAME;
        info!("Calling method '{method_name}' with request ID: {req_id}");

        let req_envelope = AethernetRpcReqEnvelope::<&T::ReqRefType> {
            req_id: req_id.clone(),
            req_type: T::METHOD_NAME.into(),
            req: &req,
        };
        let serialized_req_envelope = serde_json::to_string(&req_envelope).unwrap();
        info!(
            "Sending request to Redis queue: {}",
            self.keys.rpc_request()
        );
        info!("Request envelope: {serialized_req_envelope}");

        // send request envelope
        valkey
            .lpush(self.keys.rpc_request(), &serialized_req_envelope)
            .await
            .unwrap();

        // wait for response with a timeout
        // TODO: We should tune the timeout. Maybe should be part of the AethernetRpc trait somehow with a sane default?
        // Also, we should make sure if we timeout that the request eventually is pulled from the queue as well
        // can't set TTL/expiry on individual list items, so need to think how to close this gap
        let response_key = self.keys.rpc_response(&req_id);
        info!(
            "Waiting for response on Redis queue: {} (timeout: {}s)",
            response_key,
            Self::DEFAULT_TIMEOUT_SECONDS
        );

        let maybe_serialized_response = &valkey
            .brpop(&response_key, Self::DEFAULT_TIMEOUT_SECONDS)
            .await
            .unwrap();

        match &maybe_serialized_response {
            Some([_, serialized_response]) => {
                info!("Received response: {serialized_response}");
                // First deserialize as Result<T::RepType, AethernetError> since responses are wrapped
                match serde_json::from_str::<Result<T::RepType, AethernetError>>(
                    serialized_response,
                ) {
                    Ok(result) => result, // This is already a Result<T::RepType, AethernetError>
                    Err(e) => {
                        error!("Failed to deserialize response: {e}");
                        Err(AethernetError::SerdeError(e.to_string()))
                    }
                }
            }
            None => {
                warn!("Request timed out after 1.0s waiting on {response_key}");
                Err(AethernetError::TimedOut)
            }
        }
    }

    pub async fn get<'a, T: AethernetPubsub<'a>>(&self) -> Result<T::MsgType, AethernetError> {
        let mut valkey = self.valkey.clone();

        let key = self.keys.pubsub_latest(T::MESSAGE_NAME);
        let value = valkey.get(key).await?;
        let value = value.ok_or(AethernetError::ValueNotFound)?;
        serde_json::from_str::<T::MsgType>(&value)
            .map_err(|err| AethernetError::SerdeError(err.to_string()))
    }
}
