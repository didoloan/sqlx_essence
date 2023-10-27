use darling::{FromDeriveInput,FromField};
use quote::{quote, ToTokens};
use proc_macro2::Span;
use syn::{DeriveInput, Field, Ident, Type};
use crate::models::{ColumnOpts, FieldOpts, TableOpts};

pub fn field_to_opts(field:&Field) -> FieldOpts {
    match ColumnOpts::from_field(field) {
        Ok(val) => FieldOpts { 
            pk: val.pk.unwrap_or(false),
            updatable: val.updatable.unwrap_or(false),
            searchable: val.searchable.unwrap_or(false),
            bound: val.bound.unwrap_or(false),
            noninit: val.noninit.unwrap_or(false),
            name: field.clone().ident.unwrap(),
            db_col_name: val.rename.unwrap_or(field.clone().ident.unwrap().to_string()),
            field_type: field.clone().ty,
            is_in: val.is_in.unwrap_or(false)
        },
        Err(_) => panic!("Couldn't parse field!")
    }
}

pub fn get_fields(ast: &DeriveInput) -> Vec<Field> {
    match &ast.data {
        syn::Data::Struct(val) => match &val.fields {
            syn::Fields::Named(fields) => fields.named.iter().map(|f| f.to_owned()).collect::<Vec<Field>>(),
            _ => panic!("Cant use struct with unnamed fields!")
        },
        _ => panic!("")
    }
}

pub fn get_table_attributes(ast: &DeriveInput) -> TableOpts {
    match FromDeriveInput::from_derive_input(ast) {
        Ok(opts) => opts,
        Err(_) => panic!(r"
            Attributes not complete.
        ")
    }
}

pub fn get_suffixed_ident(ident:&Ident, suffix: &str) -> Ident {
    let mut initial = ident.to_string();
    initial.push_str(suffix);
    Ident::new(initial.as_str(), Span::call_site())
}

pub fn get_type_optioned(field_type: &Type, bound:bool, for_update:bool) -> Type {
    match field_type {
        Type::Path(typ) => {
            let mut direct = typ.path.segments.to_token_stream().to_string().replace(" ", "");
            // let mut direct = typ.path.segments.;
            direct = match direct.starts_with("Option") {
                true => direct.get(7..direct.len()-1).unwrap().to_string(),
                false => direct
            };
            let direct_ident = syn::parse_str::<Type>(direct.as_str()).unwrap();

            return match direct.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" => {
                    match bound {
                        true => {
                            let mut new_type = match for_update {
                                true => quote! {#direct_ident},
                                false => quote! {Bound<#direct_ident>}
                            };

                            new_type = quote! {Option<#new_type>};
                            Type::Verbatim(quote! {#new_type})
                        },
                        false => {
                            let mut new_type = quote! {#direct_ident};
                            new_type = quote! {Option<#new_type>};
                            Type::Verbatim(quote! {#new_type})
                        }
                    }
                },
                "NaiveDate" | "NaiveTime" | "NaiveDateTime" => {
                    let mut new_type = match for_update {
                        true => quote! {#direct_ident},
                        false => quote! {Bound<#direct_ident>}
                    };
                    new_type = quote! {Option<#new_type>};
                    Type::Verbatim(quote! {#new_type})
                },
                _ => {
                    let new_type = quote! {Option<#direct_ident>};
                    Type::Verbatim(quote! {#new_type})
                }
            }
        }
        _ => panic!("OptionalStruct only works for structs :)")
    }
}