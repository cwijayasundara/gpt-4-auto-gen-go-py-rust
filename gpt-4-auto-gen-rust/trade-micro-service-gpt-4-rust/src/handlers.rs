use actix_web::{web, HttpResponse, Responder};
use crate::db::{Trade, connect};
use tokio_postgres::Error;
use postgres_types::{FromSql, ToSql};

pub async fn create_trade(trade: web::Json<Trade>) -> impl Responder {
    let client = connect().await.unwrap();

    let result = client
        .execute(
            "INSERT INTO trades (id, value, date, trader) VALUES ($1, $2, $3, $4)",
            &[&trade.id, &trade.value, &trade.date, &trade.trader],
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(trade.0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_trades() -> impl Responder {
    let client = connect().await.unwrap();

    let result = client.query("SELECT id, value, date, trader FROM trades", &[]).await;

    match result {
        Ok(rows) => {
            let trades: Vec<Trade> = rows
                .iter()
                .map(|row| Trade {
                    id: row.get(0),
                    value: row.get(1),
                    date: row.get(2),
                    trader: row.get(3),
                })
                .collect();

            HttpResponse::Ok().json(trades)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_trade_by_id(path: web::Path<i32>) -> impl Responder {
    let client = connect().await.unwrap();
    let id = path.into_inner();

    let result = client
        .query_opt("SELECT id, value, date, trader FROM trades WHERE id = $1", &[&id])
        .await;

    match result {
        Ok(Some(row)) => {
            let trade = Trade {
                id: row.get(0),
                value: row.get(1),
                date: row.get(2),
                trader: row.get(3),
            };

            HttpResponse::Ok().json(trade)
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_trade(path: web::Path<i32>, trade: web::Json<Trade>) -> impl Responder {
    let client = connect().await.unwrap();
    let id = path.into_inner();

    let result = client
        .execute(
            "UPDATE trades SET value = $1, date = $2, trader = $3 WHERE id = $4",
            &[&trade.value, &trade.date, &trade.trader, &id],
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(trade.into_inner()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_trade(path: web::Path<i32>) -> impl Responder {
    let client = connect().await.unwrap();
    let id = path.into_inner();

    let result = client
        .execute("DELETE FROM trades WHERE id = $1", &[&id])
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
