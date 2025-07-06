// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use crate::collect::{EndpointInfo, IpcInfo, IpcType};
use crate::helpers;
use quote::quote;

/// Generate the structs used for RPC including request structs, and the actual struct representing the RPC call
pub fn generate_message_structs(ipc_info: &IpcInfo) -> proc_macro2::TokenStream {
    let (derive_serde, derive_ser) = helpers::get_derive_serde();
    let mut message_structs = vec![];

    for endpoint in &ipc_info.endpoints {
        let EndpointInfo {
            endpoint_name,
            endpoint_struct,
            rep_type,
            req_struct,
            req_ref_struct,
            ..
        } = &endpoint;

        let req_struct_fields = endpoint.req_args_name_and_type_by_value();
        let (req_ref_struct_fields, req_ref_struct_lifetime_annotation) =
            endpoint.req_args_name_and_type_by_ref_with_lifetime();

        // the actual message structs are the same for pubsub and rpc
        message_structs.push(quote! {
            // The data structure for a req on the wire
            #derive_serde
            pub struct #req_struct {
                #(pub #req_struct_fields),*
            }
            // req on the wire, but some fields as refs to prevent ownership/copying issues
            #derive_ser
            pub struct #req_ref_struct #req_ref_struct_lifetime_annotation {
                #(pub #req_ref_struct_fields),*
            }
        });

        // RPC and Pubsub structs are fairly different, impl them as two branches
        match ipc_info.ipc_type {
            IpcType::Rpc => {
                message_structs.push(quote! {
                    // the struct with the Rpc info for the RPC call for internal use by the lib
                    pub struct #endpoint_struct {}
                    impl<'a> ::aethernet::AethernetRpc<'a> for #endpoint_struct {
                         const METHOD_NAME: &'static str = stringify!(#endpoint_name);
                         type ReqType = #req_struct;
                         type ReqRefType = #req_ref_struct #req_ref_struct_lifetime_annotation;
                         type RepType = #rep_type;
                    }
                })
            }
            IpcType::Pubsub => {
                message_structs.push(quote! {
                    // the struct with the Pubsub info for the Publish call for internal use by the lib
                    #derive_serde
                    pub struct #endpoint_struct {}
                    impl<'a> ::aethernet::AethernetPubsub<'a> for #endpoint_struct {
                         const MESSAGE_NAME: &'static str = stringify!(#endpoint_name);
                         type MsgType = #req_struct;
                         type MsgRefType = #req_ref_struct #req_ref_struct_lifetime_annotation;
                    }
                })
            }
        }
    }

    let struct_mod_name = {
        syn::Ident::new(
            match ipc_info.ipc_type {
                IpcType::Pubsub => "pubsub",
                IpcType::Rpc => "rpc",
            },
            proc_macro2::Span::call_site(),
        )
    };

    // final code emission
    quote! {
        mod #struct_mod_name {
            #(#message_structs)*
        }
    }
}
