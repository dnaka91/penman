use std::env;
use anyhow::Result;

static PORT: &str = "PORT";
static DATABASE_URL: &str = "DATABASE_URL";

pub struct Env {
    pub port: u16,
    pub database_url: String,
}

impl Env {
    pub fn init() -> Result<Self> {
        let port = std::env::var(PORT)
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .expect("Failed to parse Port.");
        let db_url = env::var(DATABASE_URL).expect("Database URL was not set.").to_string();

        Ok(Self {
            port: port,
            database_url: db_url,
        })
    }
}