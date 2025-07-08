// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use crate::{
    collect::{EndpointInfo, IpcInfo},
    typing::IpcArg,
};
use quote::{format_ident, quote};

/// Generate methods for the client used to invoke RPC calls on a remote server
fn generate_rpc_calls(rpc_info: &IpcInfo) -> proc_macro2::TokenStream {
    let mut calls = vec![];
    for endpoint in &rpc_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            endpoint_struct,
            rep_type,
            ..
        } = endpoint;

        let method_args = endpoint.req_args_name_and_type_by_ref();
        let req_field_names = endpoint.req_args_name_by_value();

        calls.push(quote! {
            #[doc = "client call"]
            pub async fn #endpoint_name(&self,#(#method_args),*) -> Result<#rep_type, ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::#endpoint_struct as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { #(#req_field_names),* };
                self.client.call::<rpc::#endpoint_struct>(req).await
            }
        })
    }

    quote! {
        #(#calls)*
    }
}

/// Generate the getters for the client that fetch the latest value (if it exists) for a given
/// Pubsub endpoint.
fn generate_pubsub_getters(pubsub_info: &IpcInfo) -> proc_macro2::TokenStream {
    let mut calls = vec![];
    for endpoint in &pubsub_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            endpoint_struct,
            req_args,
            ..
        } = endpoint;

        let get_method = format_ident!("get_{}", endpoint_name);

        if let [IpcArg { name, ty }] = req_args.as_slice() {
            // if the pubsub message contains only one field, then we return just that value, no struct wrapping
            calls.push(quote! {
                pub async fn #get_method(&self) -> Result<#ty, ::aethernet::AethernetError> {
                    let msg = self.client.get::<pubsub::#endpoint_struct>().await?;
                    Ok(msg.#name)
                }
            })
        } else {
            // for complex structs, just return the struct
            calls.push(quote! {
                pub async fn #get_method(&self) -> Result<<pubsub::#endpoint_struct as ::aethernet::AethernetPubsub>::MsgType, ::aethernet::AethernetError> {
                    self.client.get::<pubsub::#endpoint_struct>().await
                }
            })
        }
    }

    quote! {
        #(#calls)*
    }
}

/// generate the code for a unified RPC client and Pubsub latest value getter
pub fn generate_client_code(rpc_info: &IpcInfo, pubsub_info: &IpcInfo) -> proc_macro2::TokenStream {
    // todo assert correct types of RPC and Pubsub info

    let IpcInfo {
        invoker_struct: client_struct,
        ..
    } = rpc_info;

    let rpc_call_methods = generate_rpc_calls(rpc_info);
    let pubsub_getter_methods = generate_pubsub_getters(pubsub_info);

    quote! {
        #[derive(Clone)]
        pub struct #client_struct {
            client: ::aethernet::AethernetRpcClient,
        }

        impl #client_struct {
            pub async fn new(connection_string: &str) -> Result<Self, ::aethernet::AethernetError> {
                Ok(Self {
                    client: ::aethernet::AethernetRpcClient::new(connection_string, DEFAULT_SERVICE_NAME, INTERFANCE_NAME).await?,
                })
            }

            #rpc_call_methods

            #pubsub_getter_methods
        }
    }
}
