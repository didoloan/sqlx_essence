mod models;
mod helpers;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Ident, Type};
use helpers::{field_to_opts, get_fields, get_suffixed_ident, get_table_attributes, get_type_optioned};
use models::{FieldOpts, SqlOpts, TableOpts};



trait OrmCreate {
    fn validate_pk(&self);
    fn create_create_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream;
    fn create_search_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream;
    fn create_update_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream;
}

impl OrmCreate for Vec<FieldOpts> {
    fn validate_pk(&self) {
        let pk_field_count = self.iter().filter(|x| x.pk).count();
        if pk_field_count > 1 {
            panic!("More than one primary key!");
        }
    }

    fn create_create_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = get_suffixed_ident(&opts.root, "Create");
        let db_ident = &opts.db;
        let concerned_fields = self.iter().filter(|fld| !fld.noninit).collect::<Vec<&FieldOpts>>();

        let idents = concerned_fields.iter().map(|f| f.name.to_owned()).collect::<Vec<Ident>>();
        let types = concerned_fields.iter().map(|f| f.field_type.to_owned()).collect::<Vec<Type>>();

        let keys_str = concerned_fields.iter().map(|x| x.db_col_name.as_str()).collect::<Vec<&str>>().join(",");

        let query = format!("INSERT INTO {}({}) VALUES ({});", opts.table, keys_str, vec!["?"; idents.len()].join(","));

        quote::quote!{
            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {
                pub async fn insert(&self, conn:&Pool<#db_ident>) -> Result<u64, std::io::Error> {
                    let mut query_res = sqlx::query(#query)
                    #(.bind(self.#idents.clone()))*
                    .execute(conn).await;
                    return match query_res {
                        Ok(res) => Ok(res.last_insert_id()),
                        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))
                    }        
                }
            }
        }
    }

    fn create_search_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let root_ident = &opts.root;
        let ident = get_suffixed_ident(&opts.root, "Search");

        let arguments_ident = &opts.arguments;

        let db_ident = &opts.db;
        let concerned_fields = self.iter().filter(|fld| fld.searchable).collect::<Vec<&FieldOpts>>();

        let retrieved_fields_str = self.iter().map(|x| if x.name.to_string() == x.db_col_name {x.db_col_name.clone()} else {format!("{} as {}", x.db_col_name, x.name.to_string())}).collect::<Vec<String>>().join(", ");

        let idents = concerned_fields.iter().map(|f| f.name.to_owned()).collect::<Vec<Ident>>();
        let types = concerned_fields.iter().map(|f| get_type_optioned(&f.field_type, f.bound, false)).collect::<Vec<Type>>();

        let bounds_fields = concerned_fields.iter().filter(|x| x.bound).map(|x| x.to_owned()).collect::<Vec<&FieldOpts>>();
        let matches_fields = concerned_fields.iter().filter(|x| !x.bound).map(|x| x.to_owned()).collect::<Vec<&FieldOpts>>();

        let bounds_field_ident = bounds_fields.iter().map(|x| x.name.clone()).collect::<Vec<Ident>>();
        
        let bounds_field_str = bounds_fields.iter().map(|x| x.db_col_name.clone()).collect::<Vec<String>>();
        
        let matches_field_ident = matches_fields.iter().map(|x| x.name.clone()).collect::<Vec<Ident>>();
        
        let matches_field_str = matches_fields.iter().map(|x| x.db_col_name.clone()).collect::<Vec<String>>();
        
        let mut query = format!("SELECT {} FROM ", retrieved_fields_str);
        
        query.push_str(&opts.table);
        query.push_str(" WHERE ");

        quote::quote!{

            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {

                pub fn get_query_args(&self) -> (String, #arguments_ident) {
                    let mut query_args:Vec<String> = Vec::new();
                    let mut query_appendages:Vec<String> = Vec::new();
                    let mut query_string = #query.to_string();
                    let mut msql_args = #arguments_ident::default();
    
    
                    #(
                        if let Some(val) = &self.#matches_field_ident {
                            msql_args.add(val);
                            let mut appendage = String::from("");
                            appendage.push_str(#matches_field_str);
                            appendage.push_str(" = ?");
                            query_appendages.push(appendage);
                        }
                    )*
    
                    #(
                        if let Some(val) = &self.#bounds_field_ident {
                            if val.min.is_some() && val.max.is_some() {
                                msql_args.add(val.min.unwrap());
                                msql_args.add(val.max.unwrap());
    
                                let mut appendage = String::from("(");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" BETWEEN ? AND ?)");
                                query_appendages.push(appendage);
                            }
                            if val.min.is_none() && val.max.is_some() {
                                msql_args.add(val.max.unwrap());
                                let mut appendage = String::from("");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" <= ?");
                                query_appendages.push(appendage);
                            }
                            if val.min.is_some() && val.max.is_none() {
                                msql_args.add(val.min.unwrap());
                                let mut appendage = String::from("");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" >= ?");
                                query_appendages.push(appendage);
                            }
                        }
                    )*
    
                    query_string.push_str(query_appendages.join(" AND ").as_str());
                    query_string.push_str(";");
                    (query_string, msql_args)
                }

                pub fn stream_search<'a>(&mut self, query:&'a str, args:#arguments_ident, conn: &'a Pool<#db_ident>) -> BoxStream<'a, Result<#root_ident, ::std::io::Error>> {
                                        
                    let call = sqlx::query_as_with::<#db_ident, #root_ident, #arguments_ident>(query, args).fetch(conn).map_err(|x| Error::new(ErrorKind::Other, x.to_string()));
                    return call.boxed();
                    
                }

                pub async fn search(&mut self, conn: &Pool<#db_ident>) -> Result<Vec<#root_ident>, ::std::io::Error> {
    
                    let mut query_args:Vec<String> = Vec::new();
                    let mut query_appendages:Vec<String> = Vec::new();
                    let mut query_string = #query.to_string();
                    let mut msql_args = #arguments_ident::default();
    
    
                    #(
                        if let Some(val) = &self.#matches_field_ident {
                            msql_args.add(val);
                            let mut appendage = String::from("");
                            appendage.push_str(#matches_field_str);
                            appendage.push_str(" = ?");
                            query_appendages.push(appendage);
                        }
                    )*
    
                    #(
                        if let Some(val) = &self.#bounds_field_ident {
                            if val.min.is_some() && val.max.is_some() {
                                msql_args.add(val.min.unwrap());
                                msql_args.add(val.max.unwrap());
    
                                let mut appendage = String::from("(");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" BETWEEN ? AND ?)");
                                query_appendages.push(appendage);
                            }
                            if val.min.is_none() && val.max.is_some() {
                                msql_args.add(val.max.unwrap());
                                let mut appendage = String::from("");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" <= ?");
                                query_appendages.push(appendage);
                            }
                            if val.min.is_some() && val.max.is_none() {
                                msql_args.add(val.min.unwrap());
                                let mut appendage = String::from("");
                                appendage.push_str(#bounds_field_str);
                                appendage.push_str(" >= ?");
                                query_appendages.push(appendage);
                            }
                        }
                    )*
    
                    query_string.push_str(query_appendages.join(" AND ").as_str());
                    query_string.push_str(";");
                    
                    let query_res = match sqlx::query_as_with::<#db_ident, #root_ident, #arguments_ident>(query_string.as_str(), msql_args).fetch_all(conn).await {
                        Ok(res) => Ok(res),
                        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))
                    };
    
                    query_res
                }
            }
        }
    }

    fn create_update_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = get_suffixed_ident(&opts.root, "Update");
        let db_ident = &opts.db;
        let arguments_ident = &opts.arguments;
        let table_name = &opts.table;
        let pk = &opts.pk;
        let concerned_fields = self.iter().filter(|fld| fld.updatable).collect::<Vec<&FieldOpts>>();

        // let field_attr = concerned_fields.iter().map(|f| Type::Verbatim(())).collect::<Vec<>>();
        let idents = concerned_fields.iter().map(|f| f.name.to_owned()).collect::<Vec<Ident>>();
        let types = concerned_fields.iter().map(|f| get_type_optioned(&f.field_type, f.bound, true)).collect::<Vec<Type>>();

        // let fields_str = idents.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let fields_str = concerned_fields.iter().map(|x| x.db_col_name.clone()).collect::<Vec<String>>();

        let mut query_str = "UPDATE ".to_string();

        query_str.push_str(table_name);
        query_str.push_str(" SET ");
        let suffix = format!(" WHERE {} = ?;", pk);

        quote::quote!{
            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {
                pub async fn update(&self, pk:u64, conn:&Pool<#db_ident>) -> Result<u64, std::io::Error> {
                    let mut query_string = #query_str.to_string();
                    let mut msql_args = #arguments_ident::default();
                    
                    #(
                        if let Some(val) = &self.#idents {
                            query_string.push_str(#fields_str);
                            query_string.push_str("= ?, ");
                            msql_args.add(val);
                        }
                    )*
                    query_string.pop();
                    query_string.pop();
                    query_string.push_str(#suffix);
                    msql_args.add(pk);
                    
                    let mut query_res = sqlx::query_with(query_string.as_str(), msql_args)
                    .execute(conn).await;
                    return match query_res {
                        Ok(res) => Ok(res.rows_affected()),
                        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(GenDBOperations, attributes(spec, from, table, jointable, bound, values, driver, implement))]
pub fn gencrud(input:TokenStream) -> TokenStream {

    let ast = syn::parse_macro_input!(input as DeriveInput);

    let root_ident = &ast.ident;

    let TableOpts { name, driver, traits } = get_table_attributes(&ast);
    
    let derives_dtos_derives = traits.split(",").map(|x| Ident::new(x.trim(), Span::call_site())).collect::<Vec<Ident>>();

    let derives_dtos_attr = quote::quote!{
        #[derive(#(#derives_dtos_derives,)*)]
    };

    let field_opts = get_fields(&ast).iter().map(field_to_opts).collect::<Vec<FieldOpts>>();
    let retrieved_fields_str = field_opts.iter().map(|x| if x.name.to_string() == x.db_col_name {x.db_col_name.clone()} else {format!("{} as {}", x.db_col_name, x.name.to_string())}).collect::<Vec<String>>().join(", ");
    let pk = field_opts.iter().find(|x| x.pk).unwrap().db_col_name.clone();
    let sql_opts = SqlOpts::new(&name, &driver, &root_ident, &pk);
    let db_ident = &sql_opts.db;
    field_opts.validate_pk();
    let create = field_opts.create_create_struct(&sql_opts, &derives_dtos_attr);
    let search = field_opts.create_search_struct(&sql_opts, &derives_dtos_attr);
    let update = field_opts.create_update_struct(&sql_opts, &derives_dtos_attr);

    let by_pk_query = format!("SELECT {} FROM {} WHERE {}=?;", &retrieved_fields_str, &name, &pk);
    let delete_query = format!("DELETE FROM {} WHERE {}=?;", &name, &pk);

    let res_tstrm = quote::quote!{

        impl #root_ident {
            pub async fn get_by_pk(pk: u64, conn:&Pool<#db_ident>) -> Result<Option<Self>, ::std::io::Error> {
                
                match sqlx::query_as::<#db_ident, #root_ident>(#by_pk_query).bind(pk).fetch_one(conn).await {
                    Ok(val) => Ok(Some(val)),
                    Err(e) => match e {
                        NotFound => Ok(None),
                        _ => Err(Error::new(ErrorKind::Other, e.to_string()))
                    }
                }
            }

            pub async fn delete(pk: u64, conn:&Pool<#db_ident>) -> Result<u64, ::std::io::Error> {
                sqlx::query(#delete_query).bind(pk).execute(conn).await
                .map(|x| x.rows_affected())
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
            }
        }
        
        #create

        #search

        #update
    };

    return res_tstrm.into();
}