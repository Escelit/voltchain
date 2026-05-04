use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use dotenvy::dotenv;

mod db;
mod models;
mod handlers;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting VoltChain Backend API...");

    let pool = db::init_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))

            .wrap(Logger::default())
            .wrap(cors)
            .service(handlers::health_check)
            .service(handlers::get_all_trades)
            .service(handlers::create_trade)
            .service(handlers::get_trade)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
