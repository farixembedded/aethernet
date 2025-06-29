// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

// Functions related to collecting all the information need for Aethernet code generation.
//
// Returns info with an attempt to retain original AST reference for meaningful error generation

use crate::helpers;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;

pub struct InterfaceInfo {
    pub interface_name_snake: syn::Ident,
    pub interface_name_ucamel: syn::Ident,

    pub rpc_info: IpcInfo,
    pub pubsub_info: IpcInfo,
}

#[derive(PartialEq)]
pub enum IpcType {
    Rpc,
    Pubsub,
}

/// Top level info about the IPC interface
pub struct IpcInfo {
    pub ipc_type: IpcType,
    // handlers trait for servicing incoming RPC calls/Subscription events
    pub handlers_trait: syn::Ident,
    // wrapper class that does static dispatch for the above trait
    pub handler_struct: syn::Ident,
    // optional invoker, in the case of Pubsub, this is the publisher
    // store as token stream so we can put a compiler error if it should be unused
    pub invoker_struct: syn::Ident,
    // the actual endpoints within an IPC (either RPC methods, or Pubsub messages)
    pub endpoints: Vec<EndpointInfo>,
}

// Info about a single IPC endpoint (either one RPC method, or Pubsub messages)
pub struct EndpointInfo {
    // The actual name used for methods and identification on the wire
    pub endpoint_name: syn::Ident,
    // the struct that holds meta information for the internal lib to handle dispatch
    pub endpoint_struct: syn::Ident,
    // the raw args (either RPC method args, or the fields of the Pubsub message)
    // note, everything is raw types, not refs
    // has a series of helpers to assembly them for use in different situations
    pub req_args: Vec<(syn::Pat, syn::Type)>,
    // the response type. This is only for RPC, for Pubsub this must be the unit type
    pub rep_type: syn::Type,
    // the name of the request struct that contains the same fields as the RPC method/Pubsub
    // message. This represents the wire format for these reqs/pubs
    pub req_struct: syn::Ident,
    // Same as `req_struct` but some types are held by reference so copies aren't needed
    pub req_ref_struct: syn::Ident,
}

impl EndpointInfo {
    // All req args as `name`
    pub fn req_args_name_by_value(&self) -> Vec<TokenStream> {
        self.req_args
            .iter()
            .map(|(pat, _ty)| quote! {#pat})
            .collect()
    }

    // All req args as `name: type`
    pub fn req_args_name_and_type_by_value(&self) -> Vec<TokenStream> {
        self.req_args
            .iter()
            .map(|(pat, ty)| quote! {#pat: #ty})
            .collect()
    }

    // All req args as `name: [&'a] type` where some types are passed by reference
    // returns a tuple of the fields, and a lifetime generic to apply to a struct holding them
    pub fn req_args_name_and_type_by_ref_with_lifetime(&self) -> (Vec<TokenStream>, TokenStream) {
        let mut has_ref = false;

        let fields = self
            .req_args
            .iter()
            .map(|(pat, ty)| match helpers::type_as_pass_by_ref(ty) {
                Some(ty) => {
                    has_ref = true;
                    quote! {#pat: &'a #ty}
                }
                None => quote! {#pat: #ty},
            })
            .collect();

        let lifetime_annotation = match has_ref {
            true => quote! {<'a>},
            false => quote! {},
        };

        (fields, lifetime_annotation)
    }

    // All req args as `name: [&]type` where some types are passed by reference
    pub fn req_args_name_and_type_by_ref(&self) -> Vec<TokenStream> {
        self.req_args
            .iter()
            .map(|(pat, ty)| match helpers::type_as_pass_by_ref(ty) {
                Some(ty) => quote! {#pat: &#ty},
                None => quote! {#pat: #ty},
            })
            .collect()
    }
}

/// Create a new ident with the same span as the input. Apply a function to manipulate the new name
pub fn new_ident_with_new_name(
    ident: &syn::Ident,
    name_manip: impl Fn(&str) -> String,
) -> syn::Ident {
    let new_name = name_manip(&ident.to_string());
    syn::Ident::new(&new_name, ident.span())
}

fn arg_is_mutable(arg: &syn::PatType) -> bool {
    let is_mut_binding = matches!(
        *arg.pat,
        syn::Pat::Ident(syn::PatIdent {
            mutability: Some(_),
            ..
        })
    );

    let is_mut_ref = matches!(
        *arg.ty,
        syn::Type::Reference(syn::TypeReference {
            mutability: Some(_),
            ..
        })
    );

    is_mut_binding || is_mut_ref
}

fn arg_is_reference(arg: &syn::PatType) -> bool {
    matches!(*arg.ty, syn::Type::Reference(_))
}

/// validate and extract info about all fn endpoints of either an RPC or Pubsub trait
fn get_ipc_trait_info(ipc_trait: &syn::ItemTrait) -> Result<Vec<EndpointInfo>, syn::Error> {
    let mut endpoints_info = vec![];

    for item in &ipc_trait.items {
        let ipc_fn = {
            match item {
                syn::TraitItem::Fn(rpc_fn) => rpc_fn,
                _ => {
                    return Err(syn::Error::new(
                        item.span(),
                        "Extra non-function item in RPC trait",
                    ));
                }
            }
        };

        let endpoint_name = ipc_fn.sig.ident.clone();
        let endpoint_struct =
            new_ident_with_new_name(&endpoint_name, |name| name.to_case(Case::UpperCamel));
        let req_struct = new_ident_with_new_name(&endpoint_name, |name| {
            format!("{}Req", name.to_case(Case::UpperCamel))
        });
        let req_ref_struct = format_ident!("{}Ref", req_struct);

        let mut req_args = vec![];
        // validate and collect arguments
        for arg in &ipc_fn.sig.inputs {
            let typed_arg = {
                match arg {
                    syn::FnArg::Typed(arg) => arg,
                    _ => {
                        return Err(syn::Error::new(
                            arg.span(),
                            "IPC signatures don't take a self argument. It's implicit",
                        ));
                    }
                }
            };

            if arg_is_reference(typed_arg) {
                return Err(syn::Error::new(
                    arg.span(),
                    "IPC args can not be a reference",
                ));
            }
            if arg_is_mutable(typed_arg) {
                return Err(syn::Error::new(arg.span(), "IPC args can not be mutable"));
            }
            // TODO: check arg type against an allow list of types

            req_args.push((*typed_arg.pat.clone(), *typed_arg.ty.clone()));
        }

        let rep_type = match &ipc_fn.sig.output {
            syn::ReturnType::Type(_, ty) => *ty.clone(),
            // if there is no return type, put in the unit type
            syn::ReturnType::Default => syn::parse_str("()").unwrap(),
        };

        endpoints_info.push(EndpointInfo {
            endpoint_name,
            endpoint_struct,
            req_args,
            rep_type,
            req_struct,
            req_ref_struct,
        });
    }

    Ok(endpoints_info)
}

/// Return the RPC and Pubsub trait items and also do high level validation of the mod structure
fn get_interface_traits(
    interface_mod: syn::ItemMod,
) -> Result<(syn::ItemTrait, syn::ItemTrait), syn::Error> {
    let mut rpc_trait = None;
    let mut pubsub_trait = None;

    match &interface_mod.content {
        Some((_brace, items)) => {
            for item in items {
                let trait_item = match item {
                    syn::Item::Trait(trait_item) => trait_item,
                    _ => {
                        return Err(syn::Error::new(
                            item.span(),
                            "Only 'Rpc' and 'Pubsub' traits are allowed in interface modules",
                        ));
                    }
                };

                match trait_item.ident.to_string().as_str() {
                    "Rpc" => rpc_trait = Some(trait_item.clone()),
                    "Pubsub" => pubsub_trait = Some(trait_item.clone()),
                    _ => {
                        return Err(syn::Error::new(
                            item.span(),
                            "Only 'Rpc' and 'Pubsub' traits are allowed in interface modules",
                        ));
                    }
                }
            }
        }
        None => {
            // shouldn't be able to get here as (item as ItemMod) should take care of things
            panic!("interface must be applied to a mod");
        }
    };

    let rpc_trait =
        rpc_trait.ok_or(syn::Error::new(interface_mod.span(), "Missing 'Rpc' trait"))?;
    let pubsub_trait = pubsub_trait.ok_or(syn::Error::new(
        interface_mod.span(),
        "Missing 'Pubsub' trait",
    ))?;

    Ok((rpc_trait, pubsub_trait))
}

/// Take in a module that contains an Aethernet interface info
/// reruns parsed InterfaceInfo on success, or a TolkenStream with an error on failure
pub fn gather_interface_info(interface_mod: syn::ItemMod) -> Result<InterfaceInfo, syn::Error> {
    let interface_name_snake =
        new_ident_with_new_name(&interface_mod.ident, |name| name.to_case(Case::Snake));
    let interface_name_ucamel =
        new_ident_with_new_name(&interface_mod.ident, |name| name.to_case(Case::UpperCamel));

    let (rpc_trait, pubsub_trait) = get_interface_traits(interface_mod)?;

    let rpc_endpoints = get_ipc_trait_info(&rpc_trait)?;
    let pubsub_endpoints = get_ipc_trait_info(&pubsub_trait)?;

    // TODO: any extra validation on endpoints.
    // mainly I think pubsub: return type must be unit type

    // helper to generate a struct name that starts with the interface name
    let interface_type_name = {
        let interface_name_ucamel = interface_name_ucamel.clone();
        move |suffix: &str| -> syn::Ident {
            new_ident_with_new_name(&interface_name_ucamel, |name| format!("{name}{suffix}"))
        }
    };

    // build the output
    Ok(InterfaceInfo {
        interface_name_snake,
        interface_name_ucamel,

        rpc_info: IpcInfo {
            ipc_type: IpcType::Rpc,
            handlers_trait: interface_type_name("RpcHandlers"),
            handler_struct: interface_type_name("RpcServer"),
            invoker_struct: interface_type_name("Client"),
            endpoints: rpc_endpoints,
        },
        pubsub_info: IpcInfo {
            ipc_type: IpcType::Pubsub,
            handlers_trait: interface_type_name("PubsubHandlers"),
            handler_struct: interface_type_name("Subscriber"),
            invoker_struct: interface_type_name("Publisher"),
            endpoints: pubsub_endpoints,
        },
    })
}
