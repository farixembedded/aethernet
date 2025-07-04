// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use quote::{ToTokens, quote};

const MACRO_DEBUG: bool = false;

/// break this out into a function so we disable derive serde in one place for macro debugging
pub fn get_derive_serde() -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let derive_serde = match MACRO_DEBUG {
        true => {
            quote! { #[macro_debug(note = "Debugging: would derive serde here")] }
        }
        false => {
            quote! {
                #[derive(::aethernet::_deps::serde::Serialize, ::aethernet::_deps::serde::Deserialize)]
                // we need to tell the serde derive macro to use our re-exported serde dep
                #[serde(crate = "::aethernet::_deps::serde")]
            }
        }
    };

    let derive_ser = match MACRO_DEBUG {
        true => {
            quote! { #[macro_debug(note = "Debugging: would derive deser here")] }
        }
        false => {
            quote! {
                #[derive(::aethernet::_deps::serde::Serialize)]
                // we need to tell the serde derive macro to use our re-exported serde dep
                #[serde(crate = "::aethernet::_deps::serde")]
            }
        }
    };

    (derive_serde, derive_ser)
}

/// Helper to get the T type from a syn::Type representing `Vec<T>`
fn extract_vec_inner(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.first() {
            if seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(ref args) = seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

/// Check if a type should be passed to call or publish functions by ref to prevent ownership/copy
/// issues. Returns the type to use when passing by reference (might be different than the input).
pub fn type_as_pass_by_ref(ty: &syn::Type) -> Option<syn::Type> {
    // TODO: would be better to inspect the full Type struct rather than just emitting as a string
    let type_string = ty.to_token_stream().to_string();
    let type_string: String = type_string.chars().filter(|c| !c.is_whitespace()).collect();

    if type_string.as_str() == "String" {
        Some(syn::parse_str("str").unwrap())
    } else if type_string.starts_with("Vec<") {
        let vec_inner_type = extract_vec_inner(ty).unwrap();

        Some(syn::parse_str(format!("[{}]", vec_inner_type.to_token_stream()).as_str()).unwrap())
    } else {
        None
    }
}
