use actix_files::NamedFile;
use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::error::BlockingError;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::Insertable;
use models::{Counter, Entry};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use structopt::StructOpt;

#[macro_use]
extern crate diesel;

mod cli;
mod db;
mod forms;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_counter(
	req: HttpRequest,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}

	let requested_id = req
		.match_info()
		.get("id")
		.unwrap_or("1")
		.parse::<i32>()
		.map_err(|_| HttpResponse::InternalServerError())?;

	let counter = web::block(move || {
		use self::schema::entries::dsl::*;

		let conn = pool.get().unwrap();

		let counter = match {
			use self::schema::counters::dsl::*;
			counters.find(requested_id).first::<Counter>(&conn)
		} {
			Ok(counter) => counter,
			Err(e) => return Err(e),
		};
		let counter_entries = Entry::belonging_to(&counter)
			.order(id.desc())
			.load::<Entry>(&conn);
		counter_entries
	})
	.await;

	match counter {
		Ok(counter) => Ok(HttpResponse::Ok().json(counter)),
		Err(BlockingError::Error(diesel::result::Error::NotFound)) => {
			Ok(HttpResponse::NotFound().body(""))
		}
		Err(_) => Ok(HttpResponse::InternalServerError().body("")),
	}
}

async fn get_counters(
	_req: HttpRequest,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}

	let counters = web::block(move || {
		use self::schema::counters::dsl::*;
		counters
			.order(id.desc())
			.load::<Counter>(&pool.get().unwrap())
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?;

	Ok(HttpResponse::Ok().json(counters))
}

async fn add(
	form: web::Json<forms::AddForm>,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}
	let new_entry = models::InsertEntry {
		reason: form.reason.clone(),
		created: now(),
		counter_id: form.counter_id,
	};

	let new_entry = web::block(move || {
		use self::schema::entries::dsl::*;
		let conn = pool.get().unwrap();
		diesel::insert_into(entries)
			.values(&new_entry)
			.get_result::<Entry>(&conn)
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?; // convert diesel error to http response

	Ok(HttpResponse::Created().body(
		serde_json::to_string(&new_entry)
			.map_err(|_| HttpResponse::InternalServerError().body(""))?,
	))
}

async fn edit(
	form: web::Json<forms::EditForm>,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	Ok("")
}

async fn delete(
	form: web::Json<forms::DeleteForm>,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	Ok("")
}

async fn auth(
	form: web::Json<forms::AuthForm>,
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

async fn index() -> Result<impl Responder, Error> {
	Ok(NamedFile::open("frontend/dist/index.html")
		.map_err(|_| HttpResponse::InternalServerError())?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let opt: cli::Opt = StructOpt::from_args();

	dotenv::dotenv().ok();

	if let Some(cmd) = opt.cmd {
		return cli::run(cmd).await;
	}

	let mut gen = rand::thread_rng();
	let private_key = (0..64)
		.into_iter()
		.map(|_| gen.gen::<u8>())
		.collect::<Vec<u8>>();

	let pool = db::connect().await;

	let mut addresses = std::env::vars()
		.filter(|(key, _)| key.starts_with("ADDRESS"))
		.map(|(_, val)| val)
		.collect::<Vec<String>>();
	if addresses.is_empty() {
		addresses.push("0.0.0.0".into());
	}

	let port = std::env::var("PORT").unwrap();

	let bind_addresses = addresses
		.into_iter()
		.map(|addr| format!("{}:{}", addr, port))
		.collect::<Vec<String>>();

	println!("BIND ADDRESSS: {:?}", bind_addresses);
	let mut server = HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			// .wrap(
			// 	actix_cors::Cors::new()
			// 		.allowed_origin("http://localhost:8080")
			// 		.allowed_origin("http://localhost:8081")
			// 		.allowed_header("Access-Control-Allow-Credentials")
			// 		.finish(),
			// )
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&private_key)
					.secure(false)
					.name("identity_cookie"),
			))
			.service(
				Scope::new("/api")
					.route("/counter/{id}", web::get().to(get_counter))
					.route("/counters", web::get().to(get_counters))
					.route("/add", web::post().to(add))
					.route("/auth", web::post().to(auth)),
			)
			.service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"))
			.default_service(
				// default to index file
				web::resource("")
					.route(web::get().to(index))
					// all requests that are not `GET`
					.route(
						web::route()
							.guard(guard::Not(guard::Get()))
							.to(HttpResponse::MethodNotAllowed),
					),
			)
	});

	for addr in bind_addresses {
		server = server
			.bind(&addr)
			.expect(&format!("failed to bind to {}", addr));
	}

	server.run().await
}

pub fn now() -> chrono::NaiveDateTime {
	let since_unix = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Time went backwards");
	chrono::NaiveDateTime::from_timestamp(since_unix.as_secs() as i64, 0)
}
