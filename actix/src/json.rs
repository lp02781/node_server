use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodePayload {
    pub id: String,
    pub timestamp: f64,
    pub data: SensorData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: u32,
    pub current: f32,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct DbRow {
    timestamp: f64,
    temperature: f32,
    humidity: i32,
    current: f32,
}