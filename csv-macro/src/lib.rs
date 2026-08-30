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

    for i in named_fields.iter() {
        println!("got named_field: {}", i.ident.clone().unwrap());
        println!("got named_type : {:#?}", types::classify(&i.ty));
    }

    return quote! {}.into();
}

fn gen_construction_for_type(name: &syn::Ident, ty: &syn::Type) -> TokenStream2 {
    match types::classify(ty) {
        FieldType::StringLiteral => quote::quote! {},
        FieldType::DateTime => quote::quote! {},
        FieldType::Other(val) => match val {
            Some(val) => panic!("Found unsupported type {:#?}", val),
            _ => panic!("Got some unsupported type"),
        },
    }
}
