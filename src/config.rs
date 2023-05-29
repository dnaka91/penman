use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseTypes,
    
}

#[derive(Debug, Deserialize)]
pub enum DatabaseTypes {
    MongoDB,
    PostgreSQL,
    MySQL
}
