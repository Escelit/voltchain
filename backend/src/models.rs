use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::trades;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = trades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EnergyTrade {
    pub id: Uuid,
    pub prosumer_address: String,
    pub consumer_address: String,
    pub amount_kwh: f64,
    pub price_per_kwh: f64,
    pub timestamp: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = trades)]
pub struct NewEnergyTrade {
    pub id: Uuid,
    pub prosumer_address: String,
    pub consumer_address: String,
    pub amount_kwh: f64,
    pub price_per_kwh: f64,
}

