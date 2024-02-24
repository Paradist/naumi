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
                    let field_to_bytes = fields.named.iter().rev().map(|field| {
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
                    let field_to_bytes = data_struct.fields.iter().enumerate().rev().map(|(index, _)| {
                        let index_lit = proc_macro2::Literal::usize_unsuffixed(index);
                        quote! { self.#index_lit.to_bytes(tx); }
                    });

                    let field_from_bytes = data_struct.fields.iter().enumerate().map(|(_, field)| {
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

                match &v.fields {
                    Fields::Unit => quote! {
                        #name::#variant_name => tx.push(#index),
                    },
                    Fields::Unnamed(_) => {
                        quote! {
                            #name::#variant_name( ref field ) => {
                                field.to_bytes(tx);
                                tx.push(#index);
                            }
                        }
                    },
                    Fields::Named(_) => panic!("Named fields in enum variants are not supported."),
                }
            });

            let from_variants = data_enum.variants.iter().enumerate().map(|(index, v)| {
                let variant_name = &v.ident;
                let index = index as u8;


                if !(index == vars-1 && index != 255){
                    match &v.fields {
                        Fields::Unit => quote! {
                            #index => #name::#variant_name,
                        },
                        Fields::Unnamed(fields) => {
                            let field_type = &fields.unnamed.first().unwrap().ty;
                            quote! {
                                #index => {
                                    let field = <#field_type as naumi::types::Convert>::from_bytes(rx)?;
                                    #name::#variant_name(field)
                                }
                            }
                        },
                        Fields::Named(_) => panic!("Named fields in enum variants are not supported."),
                    }
                } else {
                    match &v.fields {
                        Fields::Unit => quote! {
                            #index => #name::#variant_name,
                            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
                        },
                        Fields::Unnamed(fields) => {
                            let field_type = &fields.unnamed.first().unwrap().ty;
                            quote! {
                                #index => {
                                    let field = <#field_type as naumi::types::Convert>::from_bytes(rx)?;
                                    #name::#variant_name(field)
                                }
                                _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
                        }
                        },
                        Fields::Named(_) => panic!("Named fields in enum variants are not supported."),
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
                            if let Some(u) = rx.pop() {
                                match u as u8 {
                                    #(#from_variants)*
                                }
                            } else {
                                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
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
