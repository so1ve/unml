//! Implementation of the `Selection` derive macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Lit, Meta};

/// Core implementation of the Selection derive macro.
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse2(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error(),
    };

    let name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(&input, "Selection can only be derived for enums")
            .to_compile_error();
    };

    let mut default_variant = None;
    let mut variants_info: Vec<(syn::Ident, String)> = Vec::new();

    for variant in &data_enum.variants {
        // Check for unit variant
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                variant,
                "Selection variants must be unit variants (no fields)",
            )
            .to_compile_error();
        }

        let variant_name = &variant.ident;
        let mut id_value = None;
        let mut is_default = false;

        for attr in &variant.attrs {
            if attr.path().is_ident("default") {
                is_default = true;
            } else if attr.path().is_ident("id")
                && let Meta::NameValue(meta) = &attr.meta
                && let Expr::Lit(expr_lit) = &meta.value
                && let Lit::Str(lit_str) = &expr_lit.lit
            {
                id_value = Some(lit_str.value());
            }
        }

        let id = id_value.unwrap_or_else(|| variant_name.to_string().to_lowercase());
        variants_info.push((variant_name.clone(), id));

        if is_default {
            if default_variant.is_some() {
                return syn::Error::new_spanned(
                    variant,
                    "Only one variant can be marked with #[default]",
                )
                .to_compile_error();
            }
            default_variant = Some(variant_name.clone());
        }
    }

    let default_variant = match default_variant {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(&input, "One variant must be marked with #[default]")
                .to_compile_error();
        }
    };

    let id_arms: Vec<_> = variants_info
        .iter()
        .map(|(variant, id)| {
            quote! { Self::#variant => #id }
        })
        .collect();

    let from_id_arms: Vec<_> = variants_info
        .iter()
        .map(|(variant, id)| {
            quote! { #id => Self::#variant }
        })
        .collect();

    quote! {
        impl #name {
            /// Returns the default selection (single source of truth).
            #[inline]
            pub const fn default() -> Self {
                Self::#default_variant
            }

            /// Returns the string ID for this selection.
            #[inline]
            pub const fn id(&self) -> &'static str {
                match self {
                    #(#id_arms),*
                }
            }

            /// Creates a selection from a string ID.
            /// Returns the default if the ID is not recognized.
            #[inline]
            pub fn from_id(id: &str) -> Self {
                match id {
                    #(#from_id_arms,)*
                    _ => Self::default(),
                }
            }
        }

        impl ::core::default::Default for #name {
            #[inline]
            fn default() -> Self {
                Self::default()
            }
        }
    }
}
