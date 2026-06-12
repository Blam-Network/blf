use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Lit, LitInt, Meta, Token, Variant};

use crate::helpers::DeriveInputHelpers;

fn expr_to_i64(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(int), ..
        }) => int.base10_parse().ok(),
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            expr_to_i64(&unary.expr).map(|value| -value)
        }
        _ => None,
    }
}

fn variant_discriminants<'a>(variants: impl IntoIterator<Item = &'a Variant>) -> Vec<i64> {
    let variants: Vec<_> = variants.into_iter().collect();
    let mut next = 0i64;
    let mut discriminants = Vec::with_capacity(variants.len());

    for variant in &variants {
        let value = if let Some((_, expr)) = &variant.discriminant {
            let value = expr_to_i64(expr)
                .unwrap_or_else(|| panic!("unsupported enum discriminant expression on {}", variant.ident));
            next = value + 1;
            value
        } else {
            let value = next;
            next += 1;
            value
        };
        discriminants.push(value);
    }

    discriminants
}

fn min_bits_for_member_count(count: usize) -> usize {
    if count <= 1 {
        1
    } else {
        ((count - 1) as u32).ilog2() as usize + 1
    }
}

fn parse_bits_attribute(input: &DeriveInput) -> Option<usize> {
    let attribute = input.get_attribute("bits")?;

    let bits = match &attribute.meta {
        Meta::List(list) => {
            let parsed_ints: Punctuated<LitInt, Comma> = list
                .parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated)
                .unwrap_or_else(|_| panic!("invalid #[bits(...)] on {}", input.ident));

            let size_literal = parsed_ints
                .first()
                .unwrap_or_else(|| panic!("#[bits(...)] requires a bit width on {}", input.ident));

            if size_literal.to_string().starts_with("0x") {
                usize::from_str_radix(&size_literal.to_string()[2..], 16)
                    .expect("bits value is invalid")
            } else {
                size_literal.base10_parse().expect("bits value is invalid")
            }
        }
        _ => panic!(
            "unsupported attribute type for bits on {}. Use #[bits(5)] syntax.",
            input.ident
        ),
    };

    if bits == 0 {
        panic!("#[bits(...)] on {} must be greater than zero", input.ident);
    }

    Some(bits)
}

pub fn c_enum_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("c_enum can only be derived for enums"),
    };

    if variants.is_empty() {
        panic!("c_enum cannot be derived for empty enums");
    }

    let member_count = variants.len();
    let min_bits = min_bits_for_member_count(member_count);
    let wire_bits = parse_bits_attribute(&input);

    if let Some(bits) = wire_bits {
        if bits < min_bits {
            panic!(
                "#[bits({bits})] on {name} is too small for {member_count} members (minimum {min_bits} bits)"
            );
        }
    }

    let discriminants = variant_discriminants(variants.iter());
    let member_literals: Vec<_> = discriminants
        .iter()
        .map(|value| quote! { (#value as i32) })
        .collect();

    let from_index_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_ident = &variant.ident;
        let index_lit = Literal::u32_suffixed(index as u32);
        quote! { #index_lit => Ok(Self::#variant_ident), }
    });

    let to_index_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_ident = &variant.ident;
        quote! { Self::#variant_ident => Ok(#index as u32), }
    });

    let size_in_bits_impl = wire_bits.map(|bits| {
        quote! {
            fn size_in_bits() -> usize {
                #bits
            }
        }
    });

    let bits_test = wire_bits.map(|bits| {
        let test_name = format_ident!("bits_{}", name);
        let test_mod_name = format_ident!("derive_test_c_enum_{}", name);

        quote! {
            #[cfg(test)]
            mod #test_mod_name {
                use super::*;
                use ::blf_lib::types::r#enum::c_enum;

                #[test]
                fn #test_name() {
                    assert_eq!(<#name as c_enum>::size_in_bits(), #bits);
                    assert!(#bits >= #min_bits, "wire bits {} < minimum {} for {} members", #bits, #min_bits, #member_count);
                }
            }
        }
    });

    let expanded = quote! {
        impl ::blf_lib::types::r#enum::c_enum for #name {
            const MEMBERS: &'static [i32] = &[#(#member_literals),*];

            fn from_index(index: u32) -> ::blf_lib::result::BLFLibResult<Self> {
                match index {
                    #(#from_index_arms)*
                    _ => Err(::blf_lib::result::BLFLibError::from(format!(
                        "Unexpected enum index for {}: {}",
                        stringify!(#name),
                        index
                    ))),
                }
            }

            fn to_index(&self) -> ::blf_lib::result::BLFLibResult<u32> {
                match *self {
                    #(#to_index_arms)*
                }
            }

            #size_in_bits_impl
        }

        #bits_test
    };

    expanded.into()
}
