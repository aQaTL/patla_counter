use super::schema::entries;
use serde::Serialize;
use diesel::{Insertable, Queryable};

#[derive(Serialize, Queryable)]
pub struct Entry {
    pub id: i32,
    pub reason: Option<String>,
    pub created: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct InsertEntry {
    pub reason: Option<String>,
    pub created: chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct Password {
    pub id: i32,
    pub password: [char; 32],
}