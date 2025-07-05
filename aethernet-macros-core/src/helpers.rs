// Copyright 2025 Farix Embedded LLC, studio 3e8 Inc.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

use quote::quote;

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
