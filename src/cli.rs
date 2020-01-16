use crate::{db, models, Pool};
use actix_web::web;
use diesel::RunQueryDsl;
use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
	#[structopt(subcommand)]
	pub cmd: Option<Cmd>,
}

#[derive(StructOpt)]
pub enum Cmd {
	AddPassword { new_password: String },
}

pub async fn run(cmd: Cmd) -> io::Result<()> {
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

			let pool = db::connect().await;

			let res = web::block(move || {
				let mut hasher = Sha256::new();
				hasher.input(&new_password);
				let hash = hex::encode(&hasher.result()[..]);

				{
					use crate::schema::passwords::dsl::*;
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
			Ok(())
		}
	}
}
