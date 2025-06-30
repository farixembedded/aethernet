// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::AethernetPubsub;
use crate::keys::AethernetKeys;
use crate::types::AethernetError;

use redis::AsyncTypedCommands;
use tokio::sync::mpsc;

pub struct AethernetSubscriber {
    valkey: redis::aio::MultiplexedConnection,
    sub_channel: mpsc::UnboundedReceiver<redis::PushInfo>,
    keys: AethernetKeys,
}

impl AethernetSubscriber {
    pub async fn new(connection_string: &str, service_name: &str, interface_name: &str) -> Self {
        info!(
            "Creating new subscriber for service: {service_name}, interface: {interface_name}"
        );

        // the Client needs resp3 protocol to support concurrent interface use and PushInfo updates
        let connection_string = match connection_string.contains("protocol=resp3") {
            true => connection_string.to_string(),
            false => match connection_string.contains("?") {
                true => format!("{connection_string}&protocol=resp3"),
                false => format!("{connection_string}?protocol=resp3"),
            },
        };
        info!("Connecting to Redis at: {connection_string}");

        let (valkey, sub_channel) = {
            let client = redis::Client::open(connection_string.clone()).unwrap();
            let (tx, rx) = mpsc::unbounded_channel();
            let config = redis::AsyncConnectionConfig::new().set_push_sender(tx);
            let valkey = client
                .get_multiplexed_async_connection_with_config(&config)
                .await
                .unwrap();

            (valkey, rx)
        };

        let keys = AethernetKeys::new(service_name, interface_name);
        info!(
            "Client initialized. RPC request queue: {}",
            keys.rpc_request()
        );

        Self {
            valkey,
            sub_channel,
            keys,
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

    pub async fn subscribe<'a, T: AethernetPubsub<'a>>(&self) -> Result<(), AethernetError> {
        let mut valkey = self.valkey.clone();

        let key_pubsub = self.keys.pubsub(T::MESSAGE_NAME);

        valkey.subscribe(&key_pubsub).await?;

        Ok(())
    }

    pub async fn get_one_sub_message(&mut self) -> Result<(String, String), AethernetError> {
        loop {
            let incoming = self.sub_channel.recv().await.unwrap();
            if !matches!(
                incoming.kind,
                redis::PushKind::Message | redis::PushKind::PMessage | redis::PushKind::SMessage
            ) {
                // we only care about subscription messages, skip if it's anything else
                continue;
            }

            if let [
                redis::Value::BulkString(key),
                redis::Value::BulkString(value),
            ] = incoming.data.as_slice()
            {
                let valkey_key = String::from_utf8(key.clone()).unwrap();
                let msg_json = String::from_utf8(value.clone()).unwrap();

                // trim the full redis key down to just be the name
                let msg_type = match valkey_key.rsplit_once(":") {
                    Some((_, name)) => name.to_string(),
                    None => valkey_key,
                };

                return Ok((msg_type, msg_json));
            }
        }
    }
}
