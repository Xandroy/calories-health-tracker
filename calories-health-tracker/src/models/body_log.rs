use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BodyLog {
    pub date: String,
    pub weight_kg: Option<f32>,
    pub body_fat_percent: Option<f32>,
    pub water_liters: Option<f32>,
    pub sleep_hours: Option<f32>,
    pub steps: Option<u32>,
    pub notes: Option<String>
}