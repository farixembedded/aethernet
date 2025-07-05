// Copyright 2025 Farix Embedded LLC
// Licensed under the MIT License. See LICENSE file in the project root for full license information.

//! Helper for parsing and validating argument type info from the ICD

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct IpcArg {
    pub name: syn::Ident,
    pub ty: crate::typing::AethernetType,
}

impl IpcArg {
    /// `name`
    pub fn arg_name_by_value(&self) -> TokenStream {
        self.name.to_token_stream()
    }

    /// `name: type`
    pub fn arg_name_and_type_by_value(&self) -> TokenStream {
        quote! {#(self.name): #(ty.ty)}
    }

    /// `name: type` or `name: &'a type` where some types are passed by reference
    ///
    /// returns a tuple of the fields, and an optional lifetime generic to apply to a struct holding
    /// them if passed by ref
    pub fn name_and_type_by_ref_with_lifetime(&self) -> (TokenStream, Option<TokenStream>) {
        let name = &self.name;
        match self.ty.emit_rust_reference() {
            Some(ty) => (quote! {#name: &'a #ty}, Some(quote! {<'a>})),
            None => (quote! {#name: #(self.ty)}, None),
        }
    }

    /// `name: type` or `name: &type` where some types are passed by reference. Note that type might
    /// be different in the ref case
    pub fn name_and_type_by_ref(&self) -> TokenStream {
        match self.ty.emit_rust_reference() {
            Some(ty) => quote! {#(self.name): &#ty},
            None => quote! {#(self.name): #(self.ty)},
        }
    }
}

/// Represents all valid types in Aethernet. Can be nested types as well.
#[derive(Debug, PartialEq)]
pub enum AethernetType {
    // unit type
    Unit,
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
    // Compound/container types
    Vec(Box<AethernetType>),
    Array {
        t: Box<AethernetType>,
        n: usize,
    },
    Tuple(Vec<AethernetType>),
}

impl quote::ToTokens for AethernetType {
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_syn_type().into_token_stream()
    }

    fn to_token_stream(&self) -> TokenStream {
        self.to_syn_type().to_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_syn_type().to_tokens(tokens);
    }
}

impl AethernetType {
    /// get as a syn::Type. Purely syntax sugar
    pub fn to_syn_type(&self) -> syn::Type {
        match self {
            AethernetType::Unit => syn::parse_quote!(()),
            AethernetType::U8 => syn::parse_quote!(u8),
            AethernetType::I8 => syn::parse_quote!(i8),
            AethernetType::U16 => syn::parse_quote!(u16),
            AethernetType::I16 => syn::parse_quote!(i16),
            AethernetType::U32 => syn::parse_quote!(u32),
            AethernetType::I32 => syn::parse_quote!(i32),
            AethernetType::U64 => syn::parse_quote!(u64),
            AethernetType::I64 => syn::parse_quote!(i64),
            AethernetType::F32 => syn::parse_quote!(f32),
            AethernetType::F64 => syn::parse_quote!(f64),
            AethernetType::Bool => syn::parse_quote!(bool),
            AethernetType::String => syn::parse_quote!(String),
            AethernetType::Result { t, e } => syn::parse_quote!( Result<#t, #e> ),
            AethernetType::Option(t) => syn::parse_quote!( Option<#t> ),
            AethernetType::Vec(t) => syn::parse_quote!( Vec<#t> ),
            AethernetType::Array { t, n } => syn::parse_quote!( [#t; #n] ),
            AethernetType::Tuple(t) => {
                let types = t.iter().map(|t| t.to_syn_type()).collect::<Vec<_>>();
                match types.len() {
                    1 => {
                        syn::parse_quote!( (#(types[0]),) )
                    }
                    _ => syn::parse_quote!( (#(#types),*) ),
                }
            }
        }
    }

    /// Check if a type should be passed to call or publish functions by ref to prevent ownership/copy
    /// issues. Returns the type to use when passing by reference (might be different than the input).
    pub fn emit_rust_reference(&self) -> Option<syn::Type> {
        match self {
            AethernetType::String => {
                // Strings are passed by ref as a str slice
                Some(syn::parse_quote!(str))
            }
            AethernetType::Vec(t) => {
                // Vecs are passed by ref as a slice
                Some(syn::parse_quote!( [#t]))
            }
            AethernetType::Array { t, n: _ } => {
                // Arrays are passed by ref as a slice
                Some(syn::parse_quote!( [#t]))
            }
            AethernetType::Tuple(_) => {
                // Tuples are passed by ref as themselves
                Some(self.to_syn_type())
            }
            _ => None,
        }
    }
}

/// Convert an AST type representation into a more manageable AethernetType
pub fn parse_type(ty: &syn::Type) -> Result<AethernetType, syn::Error> {
    use syn::Type;
    match ty {
        Type::Array(array) => parse_array(array),
        Type::Tuple(tuple) => parse_tuple(tuple),
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

fn parse_tuple(tuple: &syn::TypeTuple) -> Result<AethernetType, syn::Error> {
    let mut inners = vec![];

    // the unit type is just a zero member tuple
    if tuple.elems.is_empty() {
        return Ok(AethernetType::Unit);
    }

    for ty in &tuple.elems {
        inners.push(parse_type(ty)?);
    }

    Ok(AethernetType::Tuple(inners))
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
    // extract basic types (confirming there are no path args)
    if path.arguments == syn::PathArguments::None {
        match path.ident.to_string().as_str() {
            // Note: don't allow parsing of unit type
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

    match path.ident.to_string().as_str() {
        "Vec" => {
            let [inner_type] = parse_bracket_inner::<1>(path)?;
            Ok(AethernetType::Vec(inner_type.into()))
        }
        "Option" => {
            let [inner_type] = parse_bracket_inner::<1>(path)?;
            Ok(AethernetType::Option(inner_type.into()))
        }
        "Result" => {
            let [t, e] = parse_bracket_inner::<2>(path)?;
            Ok(AethernetType::Result {
                t: t.into(),
                e: e.into(),
            })
        }
        _ => Err(syn::Error::new(path.span(), "Invalid Aethernet type")),
    }
}

/// Parse an arbitrary number of inner types from brackets like `Vec<Ty1, Ty2, Ty3, ..>`
/// returns an error if the number of args doesn't match the requested
fn parse_bracket_inner<const N: usize>(
    path: &syn::PathSegment,
) -> Result<[AethernetType; N], syn::Error> {
    match &path.arguments {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args, ..
        }) => {
            // rework args into a vec so we can do all vec operations on it
            let mut aethernet_args = vec![];
            for arg in args {
                match arg {
                    syn::GenericArgument::Type(ty) => aethernet_args.push(parse_type(ty)?),
                    _ => return Err(syn::Error::new(path.span(), "Incorrect arguments")),
                }
            }

            aethernet_args
                .try_into()
                .map_err(|_| syn::Error::new(path.span(), "Incorrect number of arguments"))
        }
        _ => Err(syn::Error::new(path.span(), "Incorrect arguments")),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    // helper to parse an input string with syn, and convert it to an AethernetType (or panic on
    // failure)
    fn input_to_aethernet_type(input: &str) -> AethernetType {
        parse_type(&syn::parse_str(input).unwrap())
            .unwrap_or_else(|err| panic!("{input}: Couldn't parse: {err}"))
    }

    #[test_case("()" => AethernetType::Unit)]
    #[test_case("u8" => AethernetType::U8)]
    #[test_case("i8" => AethernetType::I8)]
    #[test_case("u16" => AethernetType::U16)]
    #[test_case("i16" => AethernetType::I16)]
    #[test_case("u32" => AethernetType::U32)]
    #[test_case("i32" => AethernetType::I32)]
    #[test_case("u64" => AethernetType::U64)]
    #[test_case("i64" => AethernetType::I64)]
    #[test_case("f32" => AethernetType::F32)]
    #[test_case("f64" => AethernetType::F64)]
    #[test_case("bool" => AethernetType::Bool)]
    #[test_case("String" => AethernetType::String)]
    fn test_primitive_type_parsing(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }

    #[test_case("[u8; -4]" ; "negative number")]
    #[test_case("[u8; 4+4]" ; "expressions")]
    #[test_case("[u8; 234adsf]" ; "malformed number")]
    #[test_case("[u8; 2usize]" ; "literal type annotation")]
    fn test_parse_array_size_failing_cases(input: &str) {
        let result = parse_type(&syn::parse_str(input).unwrap());
        assert!(result.is_err());
    }

    #[test_case("[u8; 4]" => AethernetType::Array { t: AethernetType::U8.into(), n: 4 }
        ; "lenth simple number")]
    #[test_case("[u8; 23]" => AethernetType::Array { t: AethernetType::U8.into(), n: 23 }
        ; "simple number 2")]
    #[test_case("[u8; 0]" => AethernetType::Array { t: AethernetType::U8.into(), n: 0 }
        ; "zero")]
    #[test_case("[u8; 153232963]" => AethernetType::Array { t: AethernetType::U8.into(), n: 153232963 }
        ; "fairly large number")]
    #[test_case("[u8; 1]" => AethernetType::Array { t: AethernetType::U8.into(), n: 1 }
        ; "type u8")]
    #[test_case("[u64; 1]" => AethernetType::Array { t: AethernetType::U64.into(), n: 1 }
        ; "type u64")]
    #[test_case("[String; 1]" => AethernetType::Array { t: AethernetType::String.into(), n: 1 }
        ; "type String")]
    #[test_case("[[f32; 4]; 8]" =>
        AethernetType::Array {
            t: AethernetType::Array {
                t: AethernetType::F32.into(),
                n: 4
            }.into(),
            n: 8
        }
        ; "nested")]
    #[test_case("[Vec<f32>; 8]" =>
        AethernetType::Array {
            t: AethernetType::Vec(
                AethernetType::F32.into()
            ).into(),
            n: 8
        }
        ; "nested compound")]
    fn test_parse_array_passing(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }

    #[test_case("(i32,)" =>
        AethernetType::Tuple(
            vec![
                AethernetType::I32,
            ]
        ) ; "single member tuple")]
    #[test_case("(i32, u8, String, bool)" =>
        AethernetType::Tuple(
            vec![
                AethernetType::I32,
                AethernetType::U8,
                AethernetType::String,
                AethernetType::Bool,
            ]
        ) ; "multi member tuple")]
    #[test_case("(i32, u8, String, (u16, i16))" =>
        AethernetType::Tuple(
            vec![
                AethernetType::I32,
                AethernetType::U8,
                AethernetType::String,
                AethernetType::Tuple(
                    vec![
                        AethernetType::U16,
                        AethernetType::I16,
                    ]
                ),
            ]
        ) ; "nested tuples")]
    #[test_case("(i32, Vec<String>)" =>
        AethernetType::Tuple(
            vec![
                AethernetType::I32,
                AethernetType::Vec(
                    AethernetType::String.into()
                )
            ]
        ) ; "compound nested tuples")]
    fn test_parse_tuple_passing(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }

    #[test_case("Vec<i32>" => AethernetType::Vec(Box::new(AethernetType::I32)))]
    #[test_case("Vec<bool>" => AethernetType::Vec(Box::new(AethernetType::Bool)))]
    #[test_case("Vec<Vec<i32>>" =>
        AethernetType::Vec(
            AethernetType::Vec(
                AethernetType::I32.into()
            ).into()
        ); "compound Vec")]
    #[test_case("Vec<Vec<Vec<i32>>>" =>
        AethernetType::Vec(
            AethernetType::Vec(
                AethernetType::Vec(
                    AethernetType::I32.into()
                ).into()
            ).into()
        ); "double compound Vec")]
    #[test_case("Vec<[u8; 4]>" =>
        AethernetType::Vec(
            AethernetType::Array{
                t:AethernetType::U8.into(),
                n: 4,
            }.into()
        ); "compound nested type")]
    fn test_parse_vec_passing(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }

    #[test_case("Option<u8>" => AethernetType::Option(AethernetType::U8.into()) ; "normal u8")]
    #[test_case("Option<String>" => AethernetType::Option(AethernetType::String.into()) ; "normal String")]
    #[test_case("Option<Option<f32>>" =>
        AethernetType::Option(
            AethernetType::Option(
                AethernetType::F32.into()
            ).into()
        )
        ; "nested")]
    #[test_case("Option<Vec<f32>>" =>
        AethernetType::Option(
            AethernetType::Vec(
                AethernetType::F32.into()
            ).into()
        )
        ; "nested compound")]
    fn test_option(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }

    #[test_case("Result<u8, String>" =>
        AethernetType::Result{
            t: AethernetType::U8.into(),
            e: AethernetType::String.into(),
        })]
    #[test_case("Result<Result<u8, String>, bool>" =>
        AethernetType::Result{
            t: AethernetType::Result{
                t: AethernetType::U8.into(),
                e: AethernetType::String.into(),
            }.into(),
            e: AethernetType::Bool.into(),
        }; "nested")]
    #[test_case("Result<Vec<f32>, bool>" =>
        AethernetType::Result{
            t: AethernetType::Vec(
                AethernetType::F32.into(),
            ).into(),
            e: AethernetType::Bool.into(),
        }; "nested compound")]
    fn test_result(input: &str) -> AethernetType {
        input_to_aethernet_type(input)
    }
}
