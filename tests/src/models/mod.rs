use chrono::NaiveDateTime;
use sqlx::FromRow;
use utoipa::ToSchema;
use std::io::{Error, ErrorKind};
use sqlx_essence::{GenDBOperations, create_bound, StreamExt, TryStreamExt, BoxStream};
use sqlx::{MySql, Pool, mysql::MySqlArguments, Arguments};
use serde::{Deserialize, Serialize};

create_bound!(Debug, Deserialize, Serialize, ToSchema);

#[derive(Debug, GenDBOperations, Serialize, Deserialize, FromRow)]
#[table(name="users", driver="MySql", traits="Deserialize, Serialize, FromRow")]
pub struct User {
    #[spec(searchable, bound, noninit, pk)]
    pub id: u64,
    #[spec(searchable)]
    #[serde(rename="userId")]
    pub user_id: String,
    #[spec(searchable, updatable)]
    name: Option<String>,
    #[spec(searchable, updatable)]
    role: Option<String>,
    #[spec(searchable, updatable)]
    enabled: Option<bool>,
    #[spec(noninit)]
    #[serde(rename="createdAt")]
    created_at: Option<NaiveDateTime>,
    #[spec(updatable, searchable, bound, noninit)]
    #[serde(rename="updatedAt")]
    updated_at: Option<NaiveDateTime>
}