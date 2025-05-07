use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
