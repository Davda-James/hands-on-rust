use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct URLS {
    id: String,
    short_code: String,
    full_url: String,
}
