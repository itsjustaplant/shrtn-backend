use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LinkPostModel {
    pub original_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkPostResponseModel {
    pub generated_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkGetResponseModel {
    pub original_url: String,
    pub status_code: String,
}
