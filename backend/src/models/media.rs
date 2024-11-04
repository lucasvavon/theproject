use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub provider: Provider,
    pub media_type: MediaType,
    pub year: Option<i32>,
    pub rating: Option<f32>,
    pub genres: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Provider {
    Netflix,
    PrimeVideo,
    Audible,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Movie,
    Series,
    Audiobook,
}