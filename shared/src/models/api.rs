use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub version: f32,
    pub server: String,
    pub deps: Vec<String>,
}

