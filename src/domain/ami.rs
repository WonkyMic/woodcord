use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Author {
    pub Id: String,
    pub Alias: String,
    pub Platform: String,
    pub PlatformAliasId: u64
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct AuthorRequest {
    pub Alias: String,
    pub Platform: String,
    pub PlatformAliasId: u64
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct MessageRequest {
    pub AuthorId: String,
    pub Content: String,
    pub Platform: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub Id: String,
    pub AuthorId: String,
    pub Content: String,
    pub Platform: String
}
