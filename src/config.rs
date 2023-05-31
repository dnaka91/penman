use serde::Deserialize;
use db::Providers;

#[derive(Deserialize)]
pub struct Config {
    pub database: Providers,
}

