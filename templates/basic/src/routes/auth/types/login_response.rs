use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}
