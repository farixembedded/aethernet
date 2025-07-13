// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use crate::collect::{EndpointInfo, IpcInfo, IpcType};
use quote::{format_ident, quote};

/// Generate the handler trait, which is to be implemented by the consumer of this API to handle
/// incoming events (either Rpc calls, or Subscription events)
fn generate_ipc_handlers_trait(
    ipc_info: &IpcInfo,
    provide_empty_default_impl: bool,
) -> proc_macro2::TokenStream {
    let IpcInfo {
        handlers_trait: rpc_handlers_trait,
        ..
    } = ipc_info;

    let mut trait_methods = vec![];
    for endpoint in &ipc_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            rep_type,
            ..
        } = endpoint;

        let args = endpoint.req_args_name_and_type_by_value();
        let arg_names = endpoint.req_args_name_by_value();

        let default_impl = match provide_empty_default_impl {
            true => quote! {
                {
                    // need an async block so we return a future
                    async {
                        // do a no-op borrow on all vars to snub "unused arg" warning
                        #(let _ = #arg_names;)*
                    }
                }
            },
            false => quote! { ; },
        };

        trait_methods.push(quote! {
            async fn #endpoint_name(&self, #(#args),*) -> #rep_type #default_impl
        });
    }

    quote! {
        #[::aethernet::_deps::trait_variant::make(Send)]
        pub trait #rpc_handlers_trait {
            #(#trait_methods)*
        }
    }
}

/// Generate the handler struct that takes an instance of the above handler trait create by the
/// user, and does the static dispatch on incoming events to route them to the handler.
fn generate_ipc_handler(ipc_info: &IpcInfo) -> proc_macro2::TokenStream {
    let IpcInfo {
        handler_struct: rpc_handler_struct,
        handlers_trait: rpc_handlers_trait,
        ..
    } = ipc_info;

    // generate the static dispatch for incoming RPC requests
    let mut ipc_dispatch = vec![];
    let mut pubsub_subscribe = vec![];
    for endpoint in &ipc_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            endpoint_struct,
            ..
        } = endpoint;

        let args = endpoint.req_args_name_by_value();

        // only Rpc sends a reply
        match ipc_info.ipc_type {
            IpcType::Pubsub => {
                ipc_dispatch.push(quote! {
                    stringify!(#endpoint_name) => {
                        type Msg<'a> = <pubsub::#endpoint_struct as ::aethernet::AethernetPubsub<'a>>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.#endpoint_name(#(req.#args),*).await,
                            Err(_) => (), // TODO log error
                        }
                    }
                });
            }
            IpcType::Rpc => {
                ipc_dispatch.push(quote! {
                    stringify!(#endpoint_name) => {
                        match ::aethernet::_deps::serde_json::from_value::<<rpc::#endpoint_struct as ::aethernet::AethernetRpc>::ReqType>(req_envelope.req) {
                            Ok(req) => {
                                let rep = self.handlers.#endpoint_name(#(req.#args),*).await;
                                self.agent.send_rpc_reply::<rpc::#endpoint_struct>(&req_envelope.req_id, &Ok(rep)).await;
                                Ok(())
                            },
                            Err(err) => Err(::aethernet::AethernetError::SerdeError(err.to_string())),
                        }
                    }
                });
            }
        };

        // only do subscriptions at startup for Pubsub
        if ipc_info.ipc_type == IpcType::Pubsub {
            pubsub_subscribe.push(quote! {
                agent.subscribe::<pubsub::#endpoint_struct>().await.unwrap();
            });
        }
    }

    let ipc_server_or_client = match ipc_info.ipc_type {
        IpcType::Pubsub => format_ident!("AethernetSubscriber"),
        IpcType::Rpc => format_ident!("AethernetServer"),
    };

    let handle_one_incoming_fn = match ipc_info.ipc_type {
        IpcType::Pubsub => quote! {
            async fn handle_one_incoming(&mut self) {
                let (msg_type, msg_json) = match self.agent.get_one_sub_message().await {
                    Ok(value) => value,
                    Err(_) => return, // TODO log error
                };

                match msg_type.as_str() {
                    #(#ipc_dispatch)*
                    _ => (), // ignore unknown messages. TODO maybe log this
                }
            }
        },
        IpcType::Rpc => quote! {
            async fn handle_one_incoming(&mut self) {
                let maybe_req_envelope = self.agent.wait_for_and_deserialize_next_request().await;

                let req_envelope = match maybe_req_envelope {
                    Ok(envelope) => envelope,
                    // Without an envelope we can't send an error response
                    // TODO: Should log the error locally somehow
                    Err(_) => return,
                };

                let send_result = match req_envelope.req_type.as_str() {
                    #(#ipc_dispatch)*
                    _ => Err(::aethernet::AethernetError::MethodNotImplemented),
                };

                if send_result.is_err() {
                    self.agent.send_rpc_reply::<::aethernet::AethernetError>(&req_envelope.req_id, &send_result).await;
                }
            }
        },
    };
    // TODO: pretty much just the dynamic dispatch is dynamically generated. We should be able to
    // factor that out and put everything else as static code within the main lib that calls the
    // generated dispatcher. Might get a bit tricky with taking in an RPC handler?
    quote! {
        pub struct #rpc_handler_struct<T: #rpc_handlers_trait> {
            handlers: Box<T>,
            agent: ::aethernet::#ipc_server_or_client,
        }

        impl<T: #rpc_handlers_trait  + 'static> #rpc_handler_struct<T> {
            pub async fn new(connection_string: &str, handlers: Box<T>) -> Result<Self, ::aethernet::AethernetError> {
                // for now assume the interface maps to only one service
                let agent = ::aethernet::#ipc_server_or_client::new(connection_string, DEFAULT_SERVICE_NAME, INTERFANCE_NAME).await?;

                #(#pubsub_subscribe)*

                Ok(Self {
                    handlers,
                    agent,
                })
            }

            #[must_use = "The task guard must be assigned to prevent the task from being aborted immediately"]
            pub async fn spawn_handler(connection_string: &str, handlers: Box<T>) -> Result<::aethernet::AethernetHandlerGuard, ::aethernet::AethernetError> {
                let connection_string = connection_string.to_string();
                // construct the handler outside the tokio task so it is fully ready when this function returns
                // TODO: error handling
                let handler_instance = Self::new(&connection_string, handlers).await?;

                let join_handle = ::tokio::spawn(async move {
                    let mut handler_instance = handler_instance;
                    loop {
                        // TODO: do we error handle here somehow?
                        handler_instance.handle_one_incoming().await;
                    }
                });

                Ok(::aethernet::AethernetHandlerGuard {
                    join_handle,
                })
            }

            #handle_one_incoming_fn
        }
    }
}

/// Generate the IPC handler. For Pubsub this is a subscriber for use by the client, and for Rpc
/// this is the Rpc handler for the server.
pub fn generate_handler_code(
    ipc_info: &IpcInfo,
    provide_empty_default_impl: bool,
) -> proc_macro2::TokenStream {
    let ipc_handlers_trait = generate_ipc_handlers_trait(ipc_info, provide_empty_default_impl);
    let ipc_handler = generate_ipc_handler(ipc_info);

    // final code emission
    quote! {
        #ipc_handlers_trait
        #ipc_handler
    }
}
