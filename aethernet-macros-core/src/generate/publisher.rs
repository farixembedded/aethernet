// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use crate::collect::{EndpointInfo, IpcInfo, IpcType};
use quote::quote;

/// Generate the publisher struct used by the server for publishing values
pub fn generate_publisher_code(pubsub_info: &IpcInfo) -> proc_macro2::TokenStream {
    if pubsub_info.ipc_type != IpcType::Pubsub {
        panic!("`generate_publisher_code` called on non pubsub IPC info");
    }

    let IpcInfo {
        invoker_struct: pubsub_publisher_struct,
        ..
    } = &pubsub_info;

    let mut pubsub_publishers = vec![];
    for endpoint in &pubsub_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            endpoint_struct,
            ..
        } = endpoint;

        let publish_args = endpoint.req_args_name_and_type_by_ref();
        let req_field_names = endpoint.req_args_name_by_value();

        pubsub_publishers.push(quote! {
            pub async fn #endpoint_name(&self, #(#publish_args),*) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> = <pubsub::#endpoint_struct as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { #(#req_field_names),* };
                self.server.publish::<pubsub::#endpoint_struct>(&msg).await
            }
        });
    }

    // final code emission
    quote! {
        #[derive(Clone)]
        pub struct #pubsub_publisher_struct {
            server: ::aethernet::AethernetServer,
        }

        impl #pubsub_publisher_struct {
            pub async fn new(connection_string: &str) -> Result<Self, ::aethernet::AethernetError> {
                Ok(Self {
                    server: ::aethernet::AethernetServer::new(connection_string, DEFAULT_SERVICE_NAME, INTERFANCE_NAME).await?,
                })
            }

            #(#pubsub_publishers)*
        }
    }
}
