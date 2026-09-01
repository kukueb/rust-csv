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

    let struct_name: &syn::Ident = &ast.ident;

    let fields: Fields = match ast.data {
        Struct(data) => data.fields,
        Union(_) => panic!("Unions are not supported by csv deserializer. Use structs instead"),
        Enum(_) => panic!("Enums are not supported by csv deserializer. Use structs instead"),
    };

    let named_fields = fields
        .iter()
        .filter(|field| field.ident.is_some())
        .collect::<Vec<&Field>>();

    let mut field_generators = Vec::<TokenStream2>::new();

    for (i, item) in named_fields.iter().enumerate() {
        println!("got named_field: {}", item.ident.clone().unwrap());
        println!("got named_type : {:#?}", types::classify(&item.ty));

        if let Some(idt) = &item.ident {
            field_generators.push(gen_construction_for_type(idt, &item.ty, i));
        }
    }

    let (format_parts, arg_exprs): (Vec<String>, Vec<TokenStream2>) = named_fields
        .iter()
        .filter_map(|item| gen_display_for_type(item.ident.clone(), &item.ty))
        .unzip();

    let format_string: String = format_parts.join("\n");

    return quote! {
        //
        impl<'a> #struct_name<'a> {
            pub fn from_string(string: &'a str) -> Result<Self, AppError> {
                let split: Vec<&str> = string.split(',').collect();

                let new = Self {
                        #(#field_generators)*
                };

                return Ok(new);
            }
        }
        //
        impl fmt::Display for #struct_name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //             return write!(
    //                 f,
    //                 "
    // order_id = {},
    // customer_id = {},
    // restaurant_id = {},
    // driver_id = {},
    // time_stamp={}
    // ",
    //                 self.order_id, self.customer_id, self.restaurant_id, self.driver_id, self.time_stamp.format("%Y-%m-%d %H:%M:%S")
    //             );

                return write!(f, #format_string, #(#arg_exprs),*);
            }
        }
        //
        fn parse_datetime(string: &str) -> Result<NaiveDateTime, ParseError> {
            return NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
        }
        //
                    }
    .into();
}

fn gen_display_for_type(
    name: Option<syn::Ident>,
    ty: &syn::Type,
) -> Option<(String, TokenStream2)> {
    let name = name?;
    let label = name.to_string();

    match types::classify(ty) {
        FieldType::StringLiteral => Some((format!("{} = {{}}", label), quote! {self.#name})),
        FieldType::DateTime => Some((
            format!("{} = {{}}", label),
            quote! {
                self.#name.format("%Y-%m-%d %H:%M:%S"),
            },
        )),
        FieldType::Other(_) => None,
    }
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
