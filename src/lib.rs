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

/// Helper macro to compose types into an enum.
///
/// # Examples
/// ```
/// use unite::unite;
///
/// struct A;
/// struct B;
///
/// unite! {
///     /// Combination of A, B & i32 renamed to C.
///     pub enum Together {
///         A,
///         B,
///         C = i32,
///     }
/// }
/// ```
#[proc_macro]
pub fn unite(input: TokenStream) -> TokenStream {
    // parse input
    let Enum {
        attributes,
        visibility,
        name,
        kinds,
    } = parse_macro_input!(input as Enum);

    // generate type information for all enum kinds
    let full_kinds = kinds
        .into_iter()
        .map(|Kind { name, ty }| {
            let ty = if let Some(ty) = &ty {
                quote! { #ty }
            } else {
                quote! { #name }
            };
            (name, ty)
        })
        .collect::<Vec<_>>();

    // generate enum kinds
    let kinds = full_kinds.iter().map(|(kind, ty)| quote! { #kind(#ty) });

    // generate cast functions
    let funcs = full_kinds.iter().map(|(kind, ty)| {
        let snake_case = kind.to_string().to_snake_case();

        let is_name = format_ident!("is_{}", snake_case);
        let is_doc = format!(
            "Checks whether this [`{name}`] is a [`{kind}`]({name}::{kind}).",
            name = name,
            kind = kind
        );

        let as_name = format_ident!("as_{}", snake_case);
        let as_doc = format!(
            "Attempts to cast this [`{name}`] to a reference to the underlying [`{kind}`]({name}::{kind}).",
            name = name,
            kind = kind,
        );

        let as_mut_name = format_ident!("as_{}_mut", snake_case);
        let as_mut_doc = format!(
            "Attempts to cast this [`{name}`] to a mutable reference to the underlying [`{kind}`]({name}::{kind}).",
            name = name,
            kind = kind,
        );

        quote! {
            #[doc = #is_doc]
            pub fn #is_name(&self) -> bool {
                matches!(self, #name::#kind(_))
            }

            #[doc = #as_doc]
            pub fn #as_name(&self) -> Option<&#ty> {
                if let #name::#kind(contents) = self {
                    Some(contents)
                } else {
                    None
                }
            }

            #[doc = #as_mut_doc]
            pub fn #as_mut_name(&mut self) -> Option<&mut #ty> {
                if let #name::#kind(contents) = self {
                    Some(contents)
                } else {
                    None
                }
            }
        }
    });

    // generate result enum
    let result = quote! {
        #(#attributes)*
        #visibility enum #name {
            #(#kinds),*
        }

        impl #name {
            #(#funcs)*
        }
    };

    TokenStream::from(result)
}

struct Enum {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: Ident,
    kinds: Punctuated<Kind, Token![,]>,
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        input.parse::<Token![enum]>()?;
        let name = input.parse()?;

        let inner;
        braced!(inner in input);
        let kinds = inner.parse_terminated(Kind::parse)?;

        Ok(Self {
            attributes,
            visibility,
            name,
            kinds,
        })
    }
}

struct Kind {
    name: Ident,
    ty: Option<Type>,
}

impl Parse for Kind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let ty = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { name, ty })
    }
}
