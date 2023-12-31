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

        let is_postgres = &opts.db.to_string() == "Postgres";

        let res_ident = &opts.query_res;
        
        let (idents_str_pair, types):(Vec<(&Ident, &str)>,Vec<&Type>) = self.iter().filter(|fld| !fld.noninit).map(|fld| ((&fld.name, fld.db_col_name.as_str()), &fld.field_type)).unzip();
        
        let (idents, keys_str):(Vec<&Ident>, Vec<&str>) = idents_str_pair.iter().map(|(x,y)| (*x,*y)).unzip();
        
        let pos_args = match opts.db.to_string().as_str() {
            "MySql" => vec!["?"; idents.len()].join(", "),
            "Postgres" => (1..idents.len()+1).map(|x| format!("${}", x)).collect::<Vec<_>>().join(", "),
            _ => panic!("Unsupportted SQL flavour")
        };
        
        let query = format!("INSERT INTO {}({}) VALUES ({});", opts.table, keys_str.iter().map(|x| if is_postgres { format!("\"{}\"", x) } else {x.to_string()}).collect::<Vec<_>>().join(", "), pos_args);
        
        quote::quote!{
            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {
                pub fn new(#(#idents:#types,)*) -> Self {
                    Self {
                        #(#idents,)*
                    }
                }

                pub async fn insert(&self, conn:&Pool<#db_ident>) -> Result<#res_ident, ::sqlx::Error> {
                    println!("{:?}", #query);
                    sqlx::query(#query)
                    #(.bind(self.#idents.clone()))*
                    .execute(conn).await
                }
            }
        }
    }

    fn create_search_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let root_ident = &opts.root;
        let ident = get_suffixed_ident(&opts.root, "Search");

        
        let arguments_ident = &opts.arguments;
        
        let db_ident = &opts.db;
        let is_postgres = db_ident.to_string() == "Postgres".to_owned();
        let concerned_fields = self.iter().filter(|fld| fld.searchable).collect::<Vec<&FieldOpts>>();
        let cvt_op = |x:&FieldOpts| if x.name.to_string() == x.db_col_name {format!("\"{}\"",x.db_col_name.clone())} else {format!("{}{}{} as {}{}{}", if is_postgres {"\""} else {""}, x.db_col_name, if is_postgres {"\""} else {""}, if is_postgres {"\""} else {""}, x.name.to_string(), if is_postgres {"\""} else {""})};
        let retrieved_fields_str = self.iter().map(cvt_op).collect::<Vec<String>>().join(", ");
        let (idents, types):(Vec<&Ident>, Vec<Type>) = concerned_fields.iter().map(|&f| (&f.name, get_type_optioned(&f.field_type, f.bound, false))).unzip();

        let (bounds_field_ident, bounds_field_str):(Vec<&Ident>, Vec<String>) = concerned_fields.iter().filter(|x| x.bound).map(|x| x.to_owned()).map(|x| (&x.name, if is_postgres {format!("\"{}\"", x.db_col_name.clone())} else {x.db_col_name.clone()})).unzip();

        let (matches_field_ident, matches_field_str):(Vec<&Ident>, Vec<String>) = concerned_fields.iter().filter(|x| !x.bound).map(|x| x.to_owned()).map(|x| (&x.name, if is_postgres {format!("\"{}\"", x.db_col_name.clone())} else {x.db_col_name.clone()})).unzip();

        let mut query = "SELECT ".to_string();
        query.push_str(&retrieved_fields_str);
        query.push_str(" FROM ");
        
        query.push_str(&opts.table);
        query.push_str(" WHERE ");

        let pos_arg = match opts.db.to_string().as_str() {
            "MySql" => "?",
            "Postgres" => "$",
            _ => panic!("Unsupportted SQL flavour")
        };

        quote::quote!{

            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {

                pub fn new(#(#idents:#types,)*) -> Self {
                    Self {
                        #(#idents,)*
                    }
                }
                
                /**
                 * This function should be called before stream search to get the 
                 * needed query and arguments.
                 * Necessary to avoid reference error
                 */
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
                            
                            appendage.push_str(" = ");
                            appendage.push_str(#pos_arg);
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
                                
                                appendage.push_str(" BETWEEN ");
                                appendage.push_str(#pos_arg);
                                appendage.push_str(" AND ");
                                appendage.push_str(#pos_arg);
                                appendage.push_str(")");
                                query_appendages.push(appendage);
                            }
                            if val.min.is_none() && val.max.is_some() {
                                msql_args.add(val.max.unwrap());
                                let mut appendage = String::from("");
                                
                                appendage.push_str(#bounds_field_str);
                                
                                appendage.push_str(" <= ");
                                appendage.push_str(#pos_arg);
                                query_appendages.push(appendage);
                            }
                            if val.min.is_some() && val.max.is_none() {
                                msql_args.add(val.min.unwrap());
                                let mut appendage = String::from("");
                                
                                appendage.push_str(#bounds_field_str);
                                
                                appendage.push_str(" >= ");
                                appendage.push_str(#pos_arg);
                                query_appendages.push(appendage);
                            }
                        }
                    )*
    
                    query_string.push_str(&query_appendages.join(" AND "));
                    query_string.push_str(";");

                    if #is_postgres {
                        let pos_ins_index = query_string.chars().enumerate().filter(|x| '$' == x.1).enumerate().map(|x| (x.0+1, x.1.0+1)).collect::<Vec<_>>();
                        let mut count = 0;
                        for pair in pos_ins_index {
                            query_string.insert_str(pair.1+count, pair.0.to_string().as_str());
                            count += 1;
                        }
                    }
                    (query_string, msql_args)
                }

                /**
                 * Returns a fallible stream of data from the database.
                 * Call get_query_args to get the required arguments query and query_args
                 */
                pub fn stream_search<'a>(&mut self, query:&'a str, args:#arguments_ident, conn: &'a Pool<#db_ident>) -> BoxStream<'a, Result<#root_ident, ::sqlx::Error>> {
                    let call = sqlx::query_as_with::<#db_ident, #root_ident, #arguments_ident>(query, args).fetch(conn);
                    return call.boxed();                    
                }

                pub async fn search(&mut self, conn: &Pool<#db_ident>) -> Result<Vec<#root_ident>, ::sqlx::Error> {
    
                    let (query, args) = self.get_query_args();
                    
                    sqlx::query_as_with::<#db_ident, #root_ident, #arguments_ident>(&query, args).fetch_all(conn).await
                    
                }
            }
        }
    }

    fn create_update_struct(&self, opts: &SqlOpts, att:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = get_suffixed_ident(&opts.root, "Update");
        let db_ident = &opts.db;

        let is_postgres = &db_ident.to_string() == "Postgres";
        let arguments_ident = &opts.arguments;
        let res_ident = &opts.query_res;
        let table_name = opts.table;
        let mut pk = opts.pk;

        println!("{}", pk);

        // if is_postgres {
        //     pk.insert_str(0, "\"");
        //     pk.push_str("\"");
        // }

        let pk_field = self.iter().find(|x| x.pk).unwrap().clone();
        let pk_type = &pk_field.field_type;

        let concerned_fields = self.iter().filter(|fld| fld.updatable).collect::<Vec<&FieldOpts>>();

        let (idents_and_strrep, types):(Vec<(&Ident, &str)>, Vec<Type>) = concerned_fields.iter().map(|&f| ((&f.name, f.db_col_name.as_str()), get_type_optioned(&f.field_type, true, true))).unzip();

        let (idents, fields_st):(Vec<&Ident>, Vec<&str>) = idents_and_strrep.iter().map(|(x,y)| (*x,*y)).unzip();

        let fields_str = fields_st.iter().map(|&x| if is_postgres {format!("\"{}\"", x)} else {x.to_string()}).collect::<Vec<_>>();
        println!("{:?}", fields_str);

        let mut query_str = "UPDATE ".to_string();

        let pos_arg = match opts.db.to_string().as_str() {
            "MySql" => "?",
            "Postgres" => "$",
            _ => panic!("Unsupportted SQL flavour")
        };

        query_str.push_str(table_name);
        query_str.push_str(" SET ");
        let suffix = format!(" WHERE {} = {};", pk, pos_arg);

        quote::quote!{
            #att
            pub struct #ident {
                #(
                    pub #idents: #types,
                )*
            }

            impl #ident {
                pub fn new(#(#idents:#types,)*) -> Self {
                    Self {
                        #(#idents,)*
                    }
                }

                pub async fn update(&self, prim_ky:#pk_type, conn:&Pool<#db_ident>) -> Result<#res_ident, ::sqlx::Error> {
                    let mut query_string = #query_str.to_string();
                    let mut msql_args = #arguments_ident::default();
                    
                    #(
                        if let Some(val) = &self.#idents {
                            query_string.push_str(#fields_str);
                            query_string.push_str("= ");
                            query_string.push_str(#pos_arg);
                            query_string.push_str(", ");
                            msql_args.add(val);
                        }
                    )*
                    query_string.pop();
                    query_string.pop();
                    query_string.push_str(#suffix);
                    msql_args.add(prim_ky);

                    
                    if #is_postgres {
                        let pos_ins_index = query_string.chars().enumerate().filter(|x| '$' == x.1).enumerate().map(|x| (x.0+1, x.1.0+1)).collect::<Vec<_>>();
                        let mut count = 0;
                        for pair in pos_ins_index {
                            query_string.insert_str(pair.1+count, pair.0.to_string().as_str());
                            count += 1;
                        }
                    }

                    println!("{}", query_string);
                    
                    sqlx::query_with(query_string.as_str(), msql_args)
                    .execute(conn).await
                }
            }
        }
    }
}

#[proc_macro_derive(GenDBOperations, attributes(spec, from, table, jointable, bound, values, driver, implement))]
pub fn gencrud(input:TokenStream) -> TokenStream {

    let ast = syn::parse_macro_input!(input as DeriveInput);

    let root_ident = &ast.ident;

    let TableOpts { name, driver, mut traits } = match get_table_attributes(&ast) {
        Ok(tbl_attr) => tbl_attr,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let is_postgres = &driver == "Postgres";

    traits.push_str(", Default, Debug");
    
    let derives_dtos_derives = traits.split(",").map(|x| Ident::new(x.trim(), Span::call_site())).collect::<Vec<Ident>>();

    let derives_dtos_attr = quote::quote!{
        #[derive(#(#derives_dtos_derives,)*)]
    };

    let mut field_opts = Vec::new();

    for field in get_fields(&ast) {
        match field_to_opts(&field) {
            Ok(f_opts) => {
                field_opts.push(f_opts);
            },
            Err(e) => {
                return e.with_span(&field).write_errors().into();
            }
        }
    }
    let cvt_op = |x:&FieldOpts| if x.name.to_string() == x.db_col_name {format!("\"{}\"",x.db_col_name.clone())} else {format!("{}{}{} as {}{}{}", if is_postgres {"\""} else {""}, x.db_col_name, if is_postgres {"\""} else {""}, if is_postgres {"\""} else {""}, x.name.to_string(), if is_postgres {"\""} else {""})};
    let retrieved_fields_str = field_opts.iter()
    .map(cvt_op).collect::<Vec<String>>().join(", ");
    let pk_field = field_opts.iter().find(|x| x.pk).unwrap().clone();
    let (mut pk, pk_type) = (pk_field.db_col_name.clone(), &pk_field.field_type);

    if &driver == "Postgres" {
        pk.insert_str(0, "\"");
        pk.push_str("\"");
    }

    let sql_opts = SqlOpts::new(&name, &driver, &root_ident, &pk);

    let res_ident = &sql_opts.query_res;
    let db_ident = &sql_opts.db;
    field_opts.validate_pk();
    let create = field_opts.create_create_struct(&sql_opts, &derives_dtos_attr);
    let search = field_opts.create_search_struct(&sql_opts, &derives_dtos_attr);
    let update = field_opts.create_update_struct(&sql_opts, &derives_dtos_attr);

    let pos_arg = match driver.to_string().as_str() {
        "MySql" => "?",
        "Postgres" => "$1",
        _ => panic!("Unsupportted SQL flavour")
    };

    let by_pk_query = format!("SELECT {} FROM {} WHERE {}={};", &retrieved_fields_str, &name, &pk, pos_arg);
    let delete_query = format!("DELETE FROM {} WHERE {}={};", &name, &pk, pos_arg);

    println!("{}\n{}", by_pk_query, delete_query);

    let res_tstrm = quote::quote!{

        impl #root_ident {
            pub async fn get_by_pk(pk: #pk_type, conn:&Pool<#db_ident>) -> Result<Option<Self>, ::sqlx::Error> {                
                match sqlx::query_as::<#db_ident, #root_ident>(#by_pk_query).bind(pk).fetch_one(conn).await {
                    Ok(val) => Ok(Some(val)),
                    Err(e) => match e {
                        NotFound => Ok(None),
                        _ => Err(e)
                    }
                }
            }

            pub async fn delete(pk: #pk_type, conn:&Pool<#db_ident>) -> Result<#res_ident, ::sqlx::Error> {
                sqlx::query(#delete_query).bind(pk).execute(conn).await
            }
        }
        
        #create

        #search

        #update
    };

    return res_tstrm.into();
}