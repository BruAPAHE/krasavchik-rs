use bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Developer {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub email: String,
    pub password: String,
    pub status: u8,
}
