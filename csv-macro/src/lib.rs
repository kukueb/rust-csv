use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data::{Enum, Struct, Union},
    DeriveInput, Field, Fields,
};

use crate::types::FieldType;

mod types;

#[proc_macro_derive(CsvStruct)]
pub fn csv_struct_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let fields: Fields = match ast.data {
        Struct(data) => data.fields,
        Union(_) => panic!("Unions are not supported by csv deserializer. Use structs instead"),
        Enum(_) => panic!("Enums are not supported by csv deserializer. Use structs instead"),
    };

    let named_fields = fields
        .iter()
        .filter(|field| field.ident.is_some())
        .collect::<Vec<&Field>>();

    let mut field_tokens = Vec::<TokenStream2>::new();

    for (i, item) in named_fields.iter().enumerate() {
        println!("got named_field: {}", item.ident.clone().unwrap());
        println!("got named_type : {:#?}", types::classify(&item.ty));

        if let Some(idt) = &item.ident {
            field_tokens.push(gen_construction_for_type(idt, &item.ty, i));
        }
    }

    return quote! {
    //
    impl<'a> Order<'a> {
        pub fn from_string(string: &'a str) -> Result<Self, AppError> {
            let split: Vec<&str> = string.split(',').collect();

            let new = Self {
                    #(#field_tokens)*
            };

            return Ok(new);
        }
    }
    //
            }
    .into();
}

fn gen_construction_for_type(
    name: &syn::Ident,
    ty: &syn::Type,
    split_index: usize,
) -> TokenStream2 {
    match types::classify(ty) {
        FieldType::StringLiteral => quote::quote! {
            #name : split[#split_index],
        },
        FieldType::DateTime => quote::quote! {
            #name: match parse_datetime(split[#split_index]) {
                Ok(val) => val,
                Err(_) => return Err(AppError::FileParsingError(String::from(split[#split_index]))),
            },
        },
        FieldType::Other(val) => match val {
            Some(val) => panic!("Found unsupported type {:#?}", val),
            _ => panic!("Got some unsupported type"),
        },
    }
}
