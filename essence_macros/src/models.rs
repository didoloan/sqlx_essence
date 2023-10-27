use darling::{FromDeriveInput, FromField};
use proc_macro2::Span;
use syn::{Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(table))]
pub struct TableOpts {
    pub driver: String,
    pub name: String,
    pub traits: String
}

pub struct SqlOpts<'a> {
    pub table: &'a str,
    pub arguments: Ident,
    pub db: Ident,
    pub root: &'a Ident,
    pub pk: &'a str
}

impl<'a> SqlOpts<'a> {
    pub fn new(table: &'a str, driver: &'a str, root: &'a Ident, pk: &'a str) -> Self {

        let ags = match driver {
            "MySql"|"MsSql"|"Sqlite" => {
                [driver, "Arguments"].concat()
            },
            "Postgres" => {
                ["Pg", "Arguments"].concat()
            },
            _ => panic!("Unsupported database!")
        };

        Self { 
            root,
            pk,
            table,
            arguments: Ident::new(&ags, Span::call_site()),
            db: Ident::new(driver, Span::call_site())
        }
    }
}

#[derive(Debug, Default, FromField)]
#[darling(attributes(spec, serde))]
pub struct ColumnOpts {
    pub pk: Option<bool>,
    pub updatable: Option<bool>,
    pub searchable: Option<bool>,
    pub bound: Option<bool>,
    pub noninit: Option<bool>,
    pub is_in: Option<bool>,
    pub rename: Option<String>
}

#[derive(Debug)]
pub struct FieldOpts {
    pub pk: bool,
    pub updatable: bool,
    pub searchable: bool,
    pub bound: bool,
    pub is_in: bool,
    pub noninit: bool,
    pub name: Ident,
    pub field_type: Type,
    pub db_col_name: String
}

#[allow(dead_code)]
pub struct DbOpts {
    pub table: String,
}