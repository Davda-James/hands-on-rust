use serde::{Deserialize, Serialize};

use crate::Db;

#[derive(Serialize, Deserialize)]
pub struct ShortenURLRequest {
    pub url: String,
}

#[derive(Clone)]
pub struct Config {
    pub db: Db,
    pub rc: redis::Client,
}
