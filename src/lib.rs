//! This crate provides the [`unite!`](macro.unite.html) macro to easily compose existing types into an enum.
//!
//! ```toml
//! [dependencies]
//! unite = "0.1"
//! ```
//!
//! # Usage
//! ```
//! use unite::unite;
//!
//! struct A;
//! struct B;
//!
//! unite! {
//!     enum Any { A, B, C = i32 }
//! }
//! ```

use heck::SnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Ident, Token, Type, Visibility,
};

/// Helper macro to compose existing types into an enum.
///
/// # Examples
/// ```
/// use unite::unite;
///
/// pub struct One(bool);
/// pub struct Two(i32);
/// pub struct Three(f64);
///
/// unite! {
///     /// A new enum with a variant for each struct.
///     pub enum Any { One, Two, Three }
/// }
/// ```
///
/// This expands to:
///
/// ```
/// # struct One;
/// # struct Two;
/// # struct Three;
/// pub enum Any {
///     One(One),
///     Two(Two),
///     Three(Three),
/// }
/// ```
///
/// ## Renaming
/// By default the enum variants use the same name as the type, but renaming is possible.
///
/// ```
/// # use unite::unite;
/// # struct SameName;
/// unite! {
///     enum Foo {
///         SameName,
///         Renamed = i32,
///     }
/// }
/// ```
///
/// ## Helpers
/// The generated enums come with helper functions to access their variants with ease.
/// Variant names are automatically converted into `snake_case` for the function names.
///
/// ```
/// # struct One;
/// # struct Two;
/// # struct Three;
/// # unite::unite! { enum Any { One, Two, Three } }
/// let mut any: Any;
/// # any = Any::One(One);
///
/// // checks whether the enum is a specific variant
/// let is_one: bool = any.is_one();
///
/// // attempts to cast the enum to a specific variant
/// let as_two: Option<&Two> = any.as_two();
/// let as_three_mut: Option<&mut Three> = any.as_three_mut();
/// ```
///
/// The generated enums also inherently implement [`From<Variant>`].
///
/// ```
/// # struct One(bool);
/// # struct Two(i32);
/// # struct Three(f64);
/// # unite::unite! { enum Any { One, Two, Three } }
/// let any: Any = One(true).into();
/// ```
#[proc_macro]
pub fn unite(input: TokenStream) -> TokenStream {
    // parse input
    let Enum {
        attributes,
        visibility,
        name,
        variants,
    } = parse_macro_input!(input as Enum);

    // generate type information for all enum variants
    let variants_data = variants
        .into_iter()
        .map(
            |Variant {
                 attributes,
                 name,
                 ty,
             }| {
                let ty = if let Some(ty) = &ty {
                    quote! { #ty }
                } else {
                    quote! { #name }
                };
                (attributes, name, ty)
            },
        )
        .collect::<Vec<_>>();

    // generate enum variants
    let variants = variants_data.iter().map(|(attributes, variant, ty)| {
        quote! {
            #(#attributes)*
            #variant(#ty)
        }
    });

    // generate helper functions
    let funcs = variants_data.iter().map(|(_, variant, ty)| {
        // convert name to snake case
        let snake_case = variant.to_string().to_snake_case();

        // generate is check name & doc
        let is_name = format_ident!("is_{}", snake_case);
        let is_doc = format!(
            "Checks whether this [`{name}`] is a [`{variant}`]({name}::{variant}).",
            name = name,
            variant = variant
        );

        // generate as cast name & doc
        let as_name = format_ident!("as_{}", snake_case);
        let as_doc = format!(
            "Attempts to cast this [`{name}`] to a reference to the underlying [`{variant}`]({name}::{variant}).",
            name = name,
            variant = variant,
        );

        // generate as mut cast name & doc
        let as_mut_name = format_ident!("as_{}_mut", snake_case);
        let as_mut_doc = format!(
            "Attempts to cast this [`{name}`] to a mutable reference to the underlying [`{variant}`]({name}::{variant}).",
            name = name,
            variant = variant,
        );

        quote! {
            #[doc = #is_doc]
            pub fn #is_name(&self) -> bool {
                matches!(self, #name::#variant(_))
            }

            #[doc = #as_doc]
            pub fn #as_name(&self) -> Option<&#ty> {
                if let #name::#variant(contents) = self {
                    Some(contents)
                } else {
                    None
                }
            }

            #[doc = #as_mut_doc]
            pub fn #as_mut_name(&mut self) -> Option<&mut #ty> {
                if let #name::#variant(contents) = self {
                    Some(contents)
                } else {
                    None
                }
            }
        }
    });

    // generate helper impls
    let impls = variants_data.iter().map(|(_, variant, ty)| {
        quote! {
            impl From<#ty> for #name {
                fn from(inner: #ty) -> Self {
                    Self::#variant(inner)
                }
            }
        }
    });

    // generate result enum
    let result = quote! {
        #(#attributes)*
        #visibility enum #name {
            #(#variants),*
        }

        impl #name {
            #(#funcs)*
        }

        #(#impls)*
    };

    TokenStream::from(result)
}

struct Enum {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: Ident,
    variants: Punctuated<Variant, Token![,]>,
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        input.parse::<Token![enum]>()?;
        let name = input.parse()?;

        let inner;
        braced!(inner in input);
        let variants = inner.parse_terminated(Variant::parse)?;

        Ok(Self {
            attributes,
            visibility,
            name,
            variants,
        })
    }
}

struct Variant {
    attributes: Vec<Attribute>,
    name: Ident,
    ty: Option<Type>,
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;
        let ty = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self {
            attributes,
            name,
            ty,
        })
    }
}
