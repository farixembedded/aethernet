// Copyright 2025 Farix Embedded LLC
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

//! Helper for parsing and validating argument type info for the ICD

use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Debug)]
pub enum AethernetType {
    // integers
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    // floating point
    F32,
    F64,
    // other primitive types
    Bool,
    String,
    // Rust built-in enums
    Result {
        t: Box<AethernetType>,
        e: Box<AethernetType>,
    },
    Option(Box<AethernetType>),
    // Compound types
    Vec(Box<AethernetType>),
    Array {
        t: Box<AethernetType>,
        n: usize,
    },
    Tuple(Vec<AethernetType>),
    // TODO: user types (custom structs/enums defined in the ICD)
    Todo, // place holder, remove when done to expose any usage
}

/// Convert an AST type representation into a more manageable AethernetType
pub fn parse_type(ty: &syn::Type) -> Result<AethernetType, syn::Error> {
    use syn::Type;
    match ty {
        Type::Array(array) => parse_array(array),
        Type::Tuple(_) => Ok(AethernetType::Todo),
        Type::Path(path) => parse_path(path),
        // all others invalid
        _ => Err(syn::Error::new(ty.span(), "Invalid Aethernet Type")),
    }
}

fn parse_array(array: &syn::TypeArray) -> Result<AethernetType, syn::Error> {
    // rather than picking apart the AST for the length, convert it to a string and if it converts
    // to a uzise, it's valid.
    let len: usize = array
        .len
        .to_token_stream()
        .to_string()
        .parse()
        .map_err(|_| {
            syn::Error::new(
                array.len.span(),
                "Array sizes must be a positive number literal",
            )
        })?;

    let inner_type = Box::new(parse_type(&array.elem)?);

    Ok(AethernetType::Array {
        t: inner_type,
        n: len,
    })
}

fn parse_path(path: &syn::TypePath) -> Result<AethernetType, syn::Error> {
    if path.qself.is_some() {
        return Err(syn::Error::new(
            path.span(),
            "qualified self-type not allowed in Aethernet",
        ));
    }
    let path = &path.path;

    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return Err(syn::Error::new(
            path.span(),
            "Aethernet types can't be pathed",
        ));
    }

    let path = path.segments.first().unwrap(); // unwrap safe, we checked above
    // extract basic types
    if path.arguments == syn::PathArguments::None {
        match path.ident.to_string().as_str() {
            "u8" => return Ok(AethernetType::U8),
            "i8" => return Ok(AethernetType::I8),
            "u16" => return Ok(AethernetType::U16),
            "i16" => return Ok(AethernetType::I16),
            "u32" => return Ok(AethernetType::U32),
            "i32" => return Ok(AethernetType::I32),
            "u64" => return Ok(AethernetType::U64),
            "i64" => return Ok(AethernetType::I64),
            "f32" => return Ok(AethernetType::F32),
            "f64" => return Ok(AethernetType::F64),
            "bool" => return Ok(AethernetType::Bool),
            "String" => return Ok(AethernetType::String),
            _ => (), // fall through and keep looking below for more complex types
        }
    }

    // todo types:
    // Result
    // Option
    // Vec
    if path.ident.to_string().as_str() == "Vec" {
        eprintln!("Got a vec! {path:?}");
    }

    Ok(AethernetType::Todo)
}
