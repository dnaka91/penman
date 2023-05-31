pub mod postgres;

use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
trait SpaceRepo {}

#[async_trait]
trait BlogRepo {}

#[derive(Debug, Deserialize)]
pub enum Providers {
    MongoDB,
    PostgreSQL,
    MySQL
}

