use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyTrade {
    pub id: Uuid,
    pub prosumer_address: String,
    pub consumer_address: String,
    pub amount_kwh: f64,
    pub price_per_kwh: f64,
    pub timestamp: NaiveDateTime,
}
