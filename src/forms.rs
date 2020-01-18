use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddForm {
	pub counter_id: i32,
	pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthForm {
	pub password: String,
}

#[derive(Deserialize)]
pub struct EditForm {
	pub id: i32,
	pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteForm {
	pub id: i32,
}

#[derive(Deserialize)]
pub struct AddCounter {
	pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct EditCounter {
	pub id: i32,
	pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteCounter {
	pub id: i32,
}
