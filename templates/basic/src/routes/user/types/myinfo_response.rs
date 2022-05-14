use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyInfoResponse {
    pub name: String,
    pub age: u8,
}
