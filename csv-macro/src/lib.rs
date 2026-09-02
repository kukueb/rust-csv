use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data::{Enum, Struct, Union},
    DeriveInput, Field, Fields,
};

use crate::types::FieldType;

mod types;

#[proc_macro_derive(CsvStruct, attributes(split_index, split_skip_to, split_skip_amount))]
pub fn csv_struct_macro(item: TokenStream) -> TokenStream {
    let (struct_name, named_fields) = parse_struct_info(&item);

    let from_string: TokenStream2 = gen_from_string(&struct_name, &named_fields);
    let display: TokenStream2 = gen_whole_display(&struct_name, &named_fields);

    let result = quote! {

        #from_string
        #display

        fn parse_datetime(string: &str) -> Result<NaiveDateTime, ParseError> {
            return NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
        }

        fn parse_bool_from_str(string: &str) -> Result<bool, ParseBoolError> {
            match string.trim() {
                "1" => "true".parse::<bool>(),
                "0" => "false".parse::<bool>(),
                _ => "shit".parse::<bool>()
            }
        }
    }
    .into();

    #[cfg(feature = "debug-macro")]
    eprintln!("\n=== csv_struct_macro expanded ===\n{}\n===\n", result);

    return result;
}

fn parse_struct_info(item: &TokenStream) -> (syn::Ident, Vec<Field>) {
    let ast: DeriveInput = syn::parse(item.clone()).unwrap();

    let struct_name: syn::Ident = ast.ident;

    let fields: Fields = match ast.data {
        Struct(data) => data.fields,
        Union(_) => panic!("Unions are not supported by csv deserializer. Use structs instead"),
        Enum(_) => panic!("Enums are not supported by csv deserializer. Use structs instead"),
    };

    let named_fields = fields
        .iter()
        .filter(|field| field.ident.is_some())
        .map(|field| field.clone())
        .collect::<Vec<Field>>();

    return (struct_name, named_fields);
}

fn get_split_arg(item: &Field) -> Option<types::SplitType> {
    for attr in item.attrs.clone() {
        if attr.path().is_ident("split_index") {
            let parsed: syn::LitInt = attr.parse_args().unwrap();
            let uval: usize = parsed.base10_parse().unwrap();
            return Some(types::SplitType::SplitIndex(uval));
        } else if attr.path().is_ident("split_skip_to") {
            let parsed: syn::LitInt = attr.parse_args().unwrap();
            let uval: usize = parsed.base10_parse().unwrap();
            return Some(types::SplitType::SplitSkip(uval));
        } else if attr.path().is_ident("split_skip_amount") {
            let parsed: syn::LitInt = attr.parse_args().unwrap();
            let uval: usize = parsed.base10_parse().unwrap();
            return Some(types::SplitType::SplitSkipAmount(uval));
        }
    }
    return None;
}

fn gen_from_string(struct_name: &syn::Ident, named_fields: &Vec<Field>) -> TokenStream2 {
    // let field_generators: Vec<TokenStream2> = named_fields
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, item)| {
    //         let arg = get_split_arg(&item);
    //         if let Some(arg_val) = arg {
    //             match arg_val {
    //                 types::SplitType::SplitIndex(val) => {
    //                     gen_construction_for_type(item.ident.clone(), &item.ty, i)
    //                 }
    //                 types::SplitType::SplitSkip(val) => {
    //                     gen_construction_for_type(item.ident.clone(), &item.ty, val)
    //                 }
    //             }
    //         }
    //     })
    //     .collect();

    let mut field_generators: Vec<TokenStream2> = Vec::<TokenStream2>::new();

    let mut i: usize = 0;
    for item in named_fields {
        let arg = get_split_arg(item);

        if let Some(arg_val) = arg {
            match arg_val {
                types::SplitType::SplitIndex(val) => {
                    if let Some(con) = gen_construction_for_type(item.ident.clone(), &item.ty, val)
                    {
                        field_generators.push(con);
                        i += 1;
                    }
                }
                types::SplitType::SplitSkip(val) => {
                    if let Some(con) = gen_construction_for_type(item.ident.clone(), &item.ty, val)
                    {
                        field_generators.push(con);
                        i = val + 1;
                    }
                }
                types::SplitType::SplitSkipAmount(val) => {
                    if let Some(con) =
                        gen_construction_for_type(item.ident.clone(), &item.ty, i + val)
                    {
                        field_generators.push(con);
                        i += val + 1;
                    }
                }
            }
        } else {
            if let Some(con) = gen_construction_for_type(item.ident.clone(), &item.ty, i) {
                field_generators.push(con);
            }
            i += 1;
        }
    }

    return quote! {
        impl<'a> #struct_name<'a> {
            pub fn from_string(string: &'a str) -> Result<Self, AppError> {
                let split: Vec<&str> = string.split(',').collect();

                let new = Self {
                        #(#field_generators),*
                };

                return Ok(new);
            }
        }
    };
}

fn gen_whole_display(struct_name: &syn::Ident, named_fields: &Vec<Field>) -> TokenStream2 {
    let (format_parts, arg_exprs): (Vec<String>, Vec<TokenStream2>) = named_fields
        .iter()
        .filter_map(|item| gen_display_for_type(item.ident.clone(), &item.ty))
        .unzip();

    let format_string: String = format_parts.join("\n");

    return quote! {
        impl fmt::Display for #struct_name<'_> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                   return write!(f, #format_string, #(#arg_exprs),*);
                }
            }
    };
}

fn gen_display_for_type(
    name: Option<syn::Ident>,
    ty: &syn::Type,
) -> Option<(String, TokenStream2)> {
    let name = name?;
    let label = name.to_string();

    let plain_parse: Option<(String, TokenStream2)> =
        Some((format!("{} = {{}}", label), quote! {self.#name}));

    match types::classify(ty) {
        FieldType::StringLiteral => plain_parse,
        FieldType::Boolean => plain_parse,

        FieldType::DateTime => Some((
            format!("{} = {{}}", label),
            quote! {
                self.#name.format("%Y-%m-%d %H:%M:%S")
            },
        )),
        FieldType::Other(_) => None,
    }
}

fn gen_construction_for_type(
    name: Option<syn::Ident>,
    ty: &syn::Type,
    split_index: usize,
) -> Option<TokenStream2> {
    let real_name: syn::Ident;

    match name {
        Some(val) => real_name = val,
        _ => return None,
    }

    //
    let plain_parse: Option<TokenStream2> = Some(quote::quote! {
        #real_name : split[#split_index]
    });

    match types::classify(ty) {
        FieldType::StringLiteral => plain_parse,

        FieldType::Boolean => Some(quote! {
            #real_name: parse_bool_from_str(split[#split_index]).expect("Cannot parse a bool from csv")
        }),

        FieldType::DateTime => Some(quote::quote! {
            #real_name: match parse_datetime(split[#split_index]) {
                Ok(val) => val,
                Err(_) => return Err(AppError::FileParsingError(String::from(split[#split_index]))),
            }
        }),

        FieldType::Other(val) => match val {
            Some(val) => panic!("Found unsupported type {:#?}", val),
            _ => panic!("Got some unsupported type"),
        },
    }
}
