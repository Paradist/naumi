extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::*;

#[proc_macro_derive(NaumiConvert)]
pub fn convert(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_to_bytes = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        quote! {
                            self.#field_name.to_bytes(tx);
                        }
                    });

                    let field_from_bytes = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        quote! {
                            #field_name: <#field_type as naumi::types::Convert>::from_bytes(rx)?,
                        }
                    });

                    quote! {
                        impl naumi::types::Convert for #name {
                            fn to_bytes(&self, tx: &mut Vec<u8>) { #(#field_to_bytes)* }
                            fn from_bytes(rx: &mut Vec<u8>) -> std::io::Result<Self> {
                                Ok(Self { #(#field_from_bytes)* })
                            }
                        }
                }
            },
                Fields::Unnamed(_) => {
                    let field_to_bytes = data_struct.fields.iter().enumerate().map(|(index, _)| {
                        quote! { self.#index.to_bytes(tx); }
                    });

                    let field_from_bytes = data_struct.fields.iter().enumerate().map(|(index, field)| {
                        let field_type = &field.ty;
                        quote! { <#field_type as naumi::types::Convert>::from_bytes(rx)? }
                    });

                    quote! {
                        impl naumi::types::Convert for #name {
                            fn to_bytes(&self, tx: &mut Vec<u8>) { #(#field_to_bytes)* }
                            fn from_bytes(rx: &mut Vec<u8>) -> std::io::Result<Self> {
                                Ok(Self( #(#field_from_bytes),* ))
                            }
                        }
                    }
                },

                Fields::Unit => {
                    panic!("Unit structs are not supported.");
                },
            }
        },
        Data::Enum(data_enum) => {
            if data_enum.variants.len() > 255 {
                panic!("Enums with more than 255 variants are not supported due to the limit of u8.");
            }
            let vars = data_enum.variants.len() as u8;

            let variants = data_enum.variants.iter().enumerate().map(|(index, v)| {
                let variant_name = &v.ident;
                let index = index as u8;
                quote! { #name::#variant_name => tx.push(#index), }
            });

            let from_variants = data_enum.variants.iter().enumerate().map(|(index, v)| {
                let variant_name = &v.ident;
                let index = index as u8;

                if index != vars-1 {
                    quote! { #index => #name::#variant_name, }
                } else {
                    quote! {
                        #index => #name::#variant_name,
                        _ => return Err(std::io::Error::from(ErrorKind::InvalidData)),
                    }
                }
            });

            quote! {
                impl naumi::types::Convert for #name {
                    fn to_bytes(&self, tx: &mut Vec<u8>) {
                        match self {
                            #(#variants)*
                        }
                    }

                    fn from_bytes(rx: &mut Vec<u8>) -> std::io::Result<Self> {
                        Ok (
                            match rx.drain(0..1).as_slice()[0] {
                                #(#from_variants)*
                            }
                        )
                    }
                }
            }
        },
        Data::Union(_) => {
            panic!("Union type not supported")
        },
    };

    TokenStream::from(expanded)
}
