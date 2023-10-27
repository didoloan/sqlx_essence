mod models;
use chrono::Utc;
use models::{User, UserCreate, UserSearch, UserUpdate, Bound};
use sqlx::MySqlPool;
use sqlx_essence::StreamExt;
use tokio;
use serde::{Serializer, ser::SerializeSeq};
use serde_json;


#[tokio::main]
async fn main() {
    let new_user = UserCreate {
        name: Some("Harriet".into()),
        enabled: Some(true),
        role: Some("ADMIN".into()),
        user_id: "harr13t".into()
    };

    let mut user_srch = UserSearch {
        // enabled: Some(1),
        // role: Some("REGULAR".into()),
        enabled: None,
        role: None,
        updated_at: None,
        id: Some(Bound {
            min: Some(1000),
            max: Some(1005)
        }),
        name: None,
        user_id: None
    };

    let needed_update = UserUpdate {
        enabled: Some(true),
        name: Some("Revita.Lized".into()),
        role: Some("ADMIN".into()),
        updated_at: Some(Utc::now().naive_local())
    };


    let conn = MySqlPool::connect("mysql://program:program@127.0.0.1/AUTHAPI").await.unwrap();

    match new_user.insert(&conn).await {
        Ok(res) => {
            println!("{:?}", res);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    };
    match User::delete(39, &conn).await {
        Ok(count) => {
            println!("{:?}",count);
        },
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    match User::get_by_pk(250, &conn).await {
        Ok(res) => {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }

    match needed_update.update(250u64, &conn).await {
        Ok(res) => {
            println!("{:?}", res);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    let (query, args) = user_srch.get_query_args();
    let mut user_strm = user_srch.stream_search(&query, args,&conn);
    let out = std::io::stdout();

    let mut ser = serde_json::Serializer::pretty(out.lock());
    let mut seq = ser.serialize_seq(None).unwrap();
    
    while let Some(Ok(usr)) = user_strm.next().await {        
        seq.serialize_element(&usr).unwrap();
    }
    seq.end().unwrap();

}

// use std::ptr;


// struct User {
//     name:String,
//     age: u8
// }

// fn main() {
//     let user = User {
//         name: "Jane".into(),
//         age: 40
//     };
    
//     let name_ptr = ptr::addr_of!(user.name);
//     let age_ptr = ptr::addr_of!(user.age);
//     let data: u32 = 42;
//     let raw_ptr = &data as *const u32;
//     let null_ptr = ptr::null() as *const u32;
    
//     // the {:p} mapping shows pointer values as hexadecimal memory addresses
//     println!("Data address: {:p}", &data);
//     println!("Raw pointer address: {:p}", raw_ptr);
//     println!("Null pointer address: {:p}", null_ptr);
//     println!("{:?}", unsafe { name_ptr.read_unaligned() });
//     println!("{:?}", unsafe { age_ptr.read_unaligned() });
// }