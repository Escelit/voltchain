use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::EnergyTrade;
use uuid::Uuid;
use chrono::Utc;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[post("/trades")]
pub async fn create_trade(trade: web::Json<EnergyTrade>) -> impl Responder {
    // In a real app, this would save to DB and possibly trigger a blockchain transaction
    HttpResponse::Created().json(trade.into_inner())
}

#[get("/trades/{id}")]
pub async fn get_trade(id: web::Path<Uuid>) -> impl Responder {
    // Mock response
    HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "prosumer_address": "GB...123",
        "consumer_address": "GB...456",
        "amount_kwh": 15.5,
        "price_per_kwh": 0.12,
        "timestamp": Utc::now().naive_utc()
    }))
}
