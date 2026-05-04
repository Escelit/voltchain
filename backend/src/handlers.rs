use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{EnergyTrade, NewEnergyTrade};
use crate::db::DbPool;
use crate::schema::trades;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[get("/trades")]
pub async fn get_all_trades(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let results = trades::table
        .load::<EnergyTrade>(&mut conn)
        .expect("Error loading trades");

    HttpResponse::Ok().json(results)
}

#[post("/trades")]
pub async fn create_trade(
    pool: web::Data<DbPool>,
    new_trade: web::Json<NewEnergyTrade>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let trade = EnergyTrade {
        id: new_trade.id,
        prosumer_address: new_trade.prosumer_address.clone(),
        consumer_address: new_trade.consumer_address.clone(),
        amount_kwh: new_trade.amount_kwh,
        price_per_kwh: new_trade.price_per_kwh,
        timestamp: Utc::now().naive_utc(),
    };

    diesel::insert_into(trades::table)
        .values(&trade)
        .execute(&mut conn)
        .expect("Error saving new trade");

    HttpResponse::Created().json(trade)
}

#[get("/trades/{id}")]
pub async fn get_trade(
    pool: web::Data<DbPool>,
    trade_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = trades::table
        .find(trade_id.into_inner())
        .first::<EnergyTrade>(&mut conn);

    match result {
        Ok(trade) => HttpResponse::Ok().json(trade),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
