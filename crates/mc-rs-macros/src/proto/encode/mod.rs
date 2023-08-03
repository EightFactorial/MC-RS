use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Meta};

mod data_enum;
mod data_struct;

/// Derive `Encode`
pub fn derive_encode(input: proc_macro::TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).expect("Unable to DeriveInput");

    match data {
        Data::Struct(data) => data_struct::encode_struct(attrs, ident, data),
        Data::Enum(data) => data_enum::encode_enum(attrs, ident, data),
        Data::Union(_) => panic!("Cannot derive `Encode` for a union"),
    }
}

fn read_fields(fields: &Fields, field_list: &mut Vec<TokenStream>) {
    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let Some(name) = &field.ident else {
                    continue;
                };

                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        crate::buffer::VarEncode::var_encode(&self.#name, buf)?;
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Encode::encode(&self.#name, buf)?;
                    });
                }
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let index = Index::from(i);

                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        crate::buffer::VarEncode::var_encode(&self.#index, buf)?;
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Encode::encode(&self.#index, buf)?;
                    });
                }
            }
        }
        Fields::Unit => {}
    }
}