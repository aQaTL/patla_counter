use crate::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

pub async fn connect() -> Pool {
	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.")
}
