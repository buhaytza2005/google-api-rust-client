use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub title: String,
    #[serde(rename = "storeCode")]
    pub store_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Locations {
    pub locations: Vec<Location>,
}

trait LocationRequest {}
