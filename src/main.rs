use actix_files::NamedFile;
use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
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

mod forms;
mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_counter(
	req: HttpRequest,
	pool: web::Data<Pool>,
	id: Identity,
) -> Result<impl Responder, Error> {
	if let None = id.identity() {
		return Ok(HttpResponse::Forbidden().body("Access denied"));
	}

	let id = req
		.match_info()
		.get("id")
		.unwrap_or("1")
		.parse::<i32>()
		.map_err(|_| HttpResponse::InternalServerError())?;

	let counter = web::block(move || {
		use self::schema::entries::dsl::*;

		let conn = pool.get().unwrap();

		let counter = {
			use self::schema::counters::dsl::*;
			counters.find(id).first::<Counter>(&conn)?
		};
		let counter_entries = Entry::belonging_to(&counter)
			.order(id.desc())
			.load::<Entry>(&conn);
		counter_entries
	})
	.await
	.map_err(|_| HttpResponse::InternalServerError())?; // convert diesel error to http response

	Ok(HttpResponse::Ok().json(counter))
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

#[derive(StructOpt)]
struct Opt {
	#[structopt(subcommand)]
	cmd: Option<Cmd>,
}

#[derive(StructOpt)]
enum Cmd {
	AddPassword { new_password: String },
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let opt: Opt = StructOpt::from_args();

	dotenv::dotenv().ok();

	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(connspec);
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.");

	if let Some(cmd) = opt.cmd {
		match cmd {
			Cmd::AddPassword { new_password } => {
				print!("accepted {}, continue? [y/N] ", new_password);
				std::io::stdout().flush()?;
				match {
					let mut b = 0u8;
					std::io::stdin()
						.read(unsafe { std::slice::from_raw_parts_mut(&mut b as *mut u8, 1) })?;
					b
				} {
					b'y' => (),
					_ => return Ok(()),
				}
				let res = web::block(move || {
					let mut hasher = Sha256::new();
					hasher.input(&new_password);
					let hash = hex::encode(&hasher.result()[..]);

					{
						use self::schema::passwords::dsl::*;
						let conn = pool.get().unwrap();
						diesel::insert_into(passwords)
							.values(&models::NewPassword {
								password_hash: hash,
							})
							.execute(&conn)
					}
				})
				.await;

				match res {
					Ok(_) => println!("Operation successful"),
					Err(e) => eprintln!("Error: {}", e),
				}
			}
		}
		return Ok(());
	}

	let mut gen = rand::thread_rng();
	let private_key = (0..64)
		.into_iter()
		.map(|_| gen.gen::<u8>())
		.collect::<Vec<u8>>();

	let bind_addr = format!(
		"{}:{}",
		std::env::var("ADDRESS").unwrap_or(String::from("0.0.0.0")),
		std::env::var("PORT").unwrap()
	);
	println!("BIND ADDRESS: {}", bind_addr);
	HttpServer::new(move || {
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
	})
	.bind(bind_addr.clone())
	.expect(&format!("failed to bind to {}", bind_addr))
	.run()
	.await
}

pub fn now() -> chrono::NaiveDateTime {
	let since_unix = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Time went backwards");
	chrono::NaiveDateTime::from_timestamp(since_unix.as_secs() as i64, 0)
}
