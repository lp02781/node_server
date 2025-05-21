use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodePayload {
    pub id: String,
    pub timestamp: f64,
    pub data: SensorData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: u32,
    pub current: f32,
}