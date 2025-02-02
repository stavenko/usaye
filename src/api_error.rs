use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub code: String,
    pub message: String,
}
