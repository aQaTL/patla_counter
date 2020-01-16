use super::schema::*;
use diesel::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Counter)]
#[table_name = "entries"]
pub struct Entry {
	pub id: i32,
	pub reason: Option<String>,
	pub created: chrono::NaiveDateTime,
	pub counter_id: i32,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "counters"]
pub struct Counter {
	pub id: i32,
	pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct InsertEntry {
	pub reason: Option<String>,
	pub created: chrono::NaiveDateTime,
	pub counter_id: i32,
}

#[derive(Serialize, Queryable)]
pub struct Password {
	pub id: i32,
	pub password: [char; 32],
}

#[derive(Insertable)]
#[table_name = "passwords"]
pub struct NewPassword {
	pub password_hash: String,
}
