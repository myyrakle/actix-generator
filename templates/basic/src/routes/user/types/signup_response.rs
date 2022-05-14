use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupResponse {
    pub access_token: String,
    pub refresh_token: String,
}
