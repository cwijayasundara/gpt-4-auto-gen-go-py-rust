use actix_web::{web, App, HttpServer};
use crate::handlers::*;

mod db;
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/trades")
                .route(web::post().to(create_trade))
                .route(web::get().to(get_trades)))
            .service(web::resource("/trades/{id}")
                .route(web::get().to(get_trade_by_id))
                .route(web::put().to(update_trade))
                .route(web::delete().to(delete_trade)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

