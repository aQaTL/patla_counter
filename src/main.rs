use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::Entry;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[macro_use]
extern crate diesel;

mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn fetch_all_rows(
	_req: HttpRequest,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}
	let all_entries = web::block(move || {
		use self::schema::entries::dsl::*;

		let conn = pool.get().unwrap();
		entries.load::<Entry>(&conn)
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?; // convert diesel error to http response

	Ok(HttpResponse::Ok().json(all_entries))
}

#[derive(Deserialize)]
struct AddForm {
	reason: Option<String>,
}

async fn add(
	form: web::Form<AddForm>,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}
	let new_entry = models::InsertEntry {
		reason: form.reason.clone(),
		created: now(),
	};

	web::block(move || {
		use self::schema::entries::dsl::*;
		let conn = pool.get().unwrap();
		diesel::insert_into(entries)
			.values(&new_entry)
			.execute(&conn)
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?; // convert diesel error to http response
	Ok(HttpResponse::SeeOther().header("Location", "/all").body(""))
}

#[derive(Deserialize)]
struct AuthForm {
	password: String,
}

async fn auth(
	form: web::Form<AuthForm>,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	let mut hasher = Sha256::new();
	hasher.input(&form.password);
	let hash = hex::encode(&hasher.result()[..]);

	let passwords_count = web::block(move || {
		use self::schema::passwords::dsl::*;
		let conn = pool.get().unwrap();
		passwords.filter(password_hash.eq(hash)).execute(&conn)
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?; // convert diesel error to http response

	if passwords_count == 0 {
		return Ok(HttpResponse::Forbidden().body("Invalid password"));
	}

	id.remember("user".into());

	Ok(HttpResponse::Ok().body("Ok"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();

	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.");

	let mut gen = rand::thread_rng();
	let private_key = (0..64)
		.into_iter()
		.map(|_| gen.gen::<u8>())
		.collect::<Vec<u8>>();

	HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&private_key)
					.secure(false)
					.name("identity_cookie"),
			))
			.route("/all", web::get().to(fetch_all_rows))
			.route("/add", web::post().to(add))
			.route("/auth", web::post().to(auth))
			.service(actix_files::Files::new("/", "frontend").index_file("index.html"))
	})
	.bind("localhost:8080")?
	.run()
	.await
}

pub fn now() -> chrono::NaiveDateTime {
	let since_unix = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Time went backwards");
	chrono::NaiveDateTime::from_timestamp(since_unix.as_secs() as i64, 0)
}
