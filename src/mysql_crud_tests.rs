use chrono::NaiveDateTime;
use sqlx::Arguments;
use core::panic;
use essence_macros::GenDBOperations;
use futures::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Postgres, Pool, mysql::{MySqlArguments, MySqlQueryResult}, postgres::{PgArguments, PgQueryResult}};
use std::default::Default;
use tokio;


create_bound!(Deserialize, Serialize);

macro_rules! create_connection {
    ($engine:ident) => {
        match prepare_connection::<$engine>().await {
            Ok(cn) => cn,
            Err(_) => {
                panic!("Couldn't create connection");
            }
        }
    };
}

#[derive(Debug, GenDBOperations, Serialize, Deserialize, FromRow)]
#[table(name = "users", driver = "Postgres", traits = "Deserialize, Serialize")]
pub struct User {
    #[spec(searchable, bound, noninit, pk)]
    pub id: i64,
    #[spec(searchable)]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[spec(searchable, updatable)]
    name: Option<String>,
    #[spec(searchable, updatable)]
    role: Option<String>,
    #[spec(searchable, updatable)]
    enabled: Option<bool>,
    #[spec(noninit)]
    #[serde(rename = "createdAt")]
    created_at: Option<NaiveDateTime>,
    #[spec(updatable, searchable, bound, noninit)]
    #[serde(rename = "updatedAt")]
    updated_at: Option<NaiveDateTime>,
}

async fn prepare_connection<DB: sqlx::Database>() -> Result<Pool<DB>, ::sqlx::Error> {
    // Ok(Pool::<DB>::connect("mysql://program:program@127.0.0.1/AUTHAPI").await?)
    Ok(Pool::<DB>::connect("postgres://postgres:mysecretpassword@localhost:5432/AUTHAPI").await?)
}

#[tokio::test]
async fn test_create() {
    let conn = create_connection!(Postgres);

    let new_user = UserCreate::new(
        "yvcwsaasdbk".into(),
        Some("hbjd sDev".into()),
        Some("ADMIN".into()),
        Some(true)
    );

    let query_res = match new_user.insert(&conn).await {
        Ok(val) => val,
        Err(e) => panic!("{:?}",e)
    };

    assert_eq!(true, true);
}

// #[tokio::test]
// async fn test_search() {
//     let conn = create_connection!(Postgres);

//     let mut search = UserSearch::new(
//         Some(Bound {
//             min: Some(1),
//             max: Some(5),
//         }),
//         None,
//         None,
//         None,
//         None,
//         None,
//     );
    
//     let search_res = search.search(&conn).await;

//     assert_eq!(search_res.is_ok(), true);
// }

// #[tokio::test]
// async fn test_search_stream() {
//     let conn = create_connection!(Postgres);

//     let mut search = UserSearch::new(
//         Some(Bound {
//             min: Some(1),
//             max: Some(5),
//         }),
//         None,
//         None,
//         None,
//         None,
//         None,
//     );
    
//     let (query, args) = search.get_query_args();

//     let stream = search.stream_search(&query, args, &conn);

//     let users_res = stream.try_collect::<Vec<_>>().await;

//     assert_eq!(users_res.is_ok(), true);
// }

// #[tokio::test]
// async fn test_by_pk() {
//     let conn = create_connection!(Postgres);

//     let search_res = User::get_by_pk(2, &conn).await;

//     println!("{:?}", search_res);

//     assert_eq!(search_res.is_ok(), true);
// }


// #[tokio::test]
// async fn test_update() {
//     let conn = create_connection!(Postgres);

//     let update = UserUpdate::new(Some("Updated Name".into()), None, None, None);
//     let update_res = update.update(8, &conn).await;

//     assert_eq!(update_res.is_ok(), true);
// }
