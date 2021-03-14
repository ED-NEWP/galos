//! # Database Setup
//!
//! ```sh
//! cargo install sqlx-cli
//! sqlx database create
//! sqlx migrate run
//! ```
//!
//! # Database Backup and Restore
//!
//! ```sh
//! pg_dump -Fc $database > $database.dump
//! sqlx database create
//! pg_restore -d $database < $database.dump
//! ```
#![feature(crate_visibility_modifier)]
use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::io::ErrorKind;

pub mod error;
pub use self::error::{Result, Error};

pub struct Database {
    crate pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        match dotenv::dotenv() {
            Err(dotenv::Error::Io(err)) if err.kind() == ErrorKind::NotFound => {
                // TODO: Add an info log here
                // info!("Unable find a dotenv file, ignoring.");
            }
            Err(err) => return Err(Error::Env(err)),
            _ => {}
        }

        let url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url).await?;

        Ok(Database { pool })
    }

    pub async fn from_url(url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url).await?;

        Ok(Database { pool })
    }
}

pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

impl Page {
    pub fn by(limit: i64) -> Self {
        Page {
            limit,
            offset: 0,
        }
    }

    pub fn turn(&self, n: i64) -> Self {
        Page {
            limit: self.limit,
            offset: self.offset + n,
        }
    }
}

pub mod articles;
pub mod systems;
pub mod factions;
