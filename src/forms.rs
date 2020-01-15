use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddForm {
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
