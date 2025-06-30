// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

#[derive(Clone)]
/// Generates the Redis key names for different parts of the IPC system.
pub struct AethernetKeys {
    global_namespace: String,
    service_name: String,
    interface_name: String,
}

impl AethernetKeys {
    pub fn new(service_name: &str, interface_name: &str) -> Self {
        Self {
            global_namespace: "aethernet".into(),
            service_name: service_name.into(),
            interface_name: interface_name.into(),
        }
    }

    pub fn pubsub(&self, pubsub_name: &str) -> String {
        let Self {
            global_namespace,
            service_name,
            interface_name,
            ..
        } = self;

        format!("{global_namespace}:{service_name}:{interface_name}:pubsub:{pubsub_name}")
    }

    pub fn pubsub_latest(&self, pubsub_name: &str) -> String {
        let Self {
            global_namespace,
            service_name,
            interface_name,
            ..
        } = self;

        format!("{global_namespace}:{service_name}:{interface_name}:pubsub:latest:{pubsub_name}")
    }

    pub fn rpc_request(&self) -> String {
        let Self {
            global_namespace,
            service_name,
            interface_name,
            ..
        } = self;

        format!("{global_namespace}:{service_name}:{interface_name}:rpc_request")
    }

    pub fn rpc_response(&self, request_id: &str) -> String {
        let Self {
            global_namespace,
            service_name,
            interface_name,
            ..
        } = self;

        format!("{global_namespace}:{service_name}:{interface_name}:rpc_response:{request_id}")
    }
}
