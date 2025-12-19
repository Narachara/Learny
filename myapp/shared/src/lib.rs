pub mod models;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse {
    pub path: String,
}
