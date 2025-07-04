// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use proc_macro::TokenStream;

// bring in re-exported deps from aethernet_macros_core
use aethernet_macros_core::deps::{quote, syn};
// bring in sub-mods of aethernet_macros_core
use aethernet_macros_core::{collect, generate};

use quote::quote;
use syn::parse_macro_input;

use collect::{InterfaceInfo, IpcInfo};
use generate::{
    generate_client_code, generate_handler_code, generate_message_structs, generate_publisher_code,
};

#[proc_macro_attribute]
pub fn interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_interface_mod = parse_macro_input!(item as syn::ItemMod);

    // collect the information about the user input (and validate at the same time)
    let interface_info = match collect::gather_interface_info(input_interface_mod) {
        Ok(value) => value,
        Err(err) => return err.into_compile_error().into(),
    };

    // generate the sub-blocks of code
    let rpc_messages_code = generate_message_structs(&interface_info.rpc_info);
    let pubsub_messages_code = generate_message_structs(&interface_info.pubsub_info);
    let rpc_handler_code = generate_handler_code(&interface_info.rpc_info, false);
    let pubsub_subscriber_code = generate_handler_code(&interface_info.pubsub_info, true);
    let publisher_code = generate_publisher_code(&interface_info.pubsub_info);
    let client_code = generate_client_code(&interface_info.rpc_info, &interface_info.pubsub_info);

    // quote can't access fields of structs and the like, so we have to destruct the fields for use
    // (this pattern is repeated throughout this crate)
    let InterfaceInfo {
        interface_name_snake,
        interface_name_ucamel: interface_name_camel,
        rpc_info,
        pubsub_info,
    } = interface_info;

    let IpcInfo {
        handlers_trait: rpc_handlers_trait,
        handler_struct: rpc_server_struct,
        invoker_struct: client_struct,
        ..
    } = &rpc_info;

    let IpcInfo {
        handlers_trait: pubsub_handlers_trait,
        handler_struct: pubsub_subscriber_struct,
        invoker_struct: pubsub_publisher_struct,
        ..
    } = &pubsub_info;

    // emit the actual code
    let output = quote! {
        mod #interface_name_snake {
            const INTERFANCE_NAME: &str = stringify!(#interface_name_camel);
            // maybe this should use the crate name (the using crate, not this crate)?
            const DEFAULT_SERVICE_NAME: &str = stringify!(#interface_name_camel);

            #rpc_messages_code
            #pubsub_messages_code
            #rpc_handler_code
            #pubsub_subscriber_code
            #publisher_code
            #client_code
        }
        // pub use to export the generated API
        #[allow(unused_imports)]
        pub use #interface_name_snake::{
            #rpc_handlers_trait,
            #rpc_server_struct,
            #pubsub_handlers_trait,
            #pubsub_subscriber_struct,
            #pubsub_publisher_struct,
            #client_struct,
        };
    };
    output.into()
}
