use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    pub state: String,
    pub databases: String,
}
