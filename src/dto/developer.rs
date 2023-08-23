use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Developer {
    pub email: String,
    pub status: u8,
    pub password: String,
}
