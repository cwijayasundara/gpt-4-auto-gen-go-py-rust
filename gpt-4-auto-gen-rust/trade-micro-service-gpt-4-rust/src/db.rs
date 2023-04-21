use tokio_postgres::{NoTls, Client, Config};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: i32,
    pub value: f64,
    pub date: NaiveDateTime,
    pub trader: String,
}

pub async fn connect() -> Result<Client, tokio_postgres::Error> {
    let mut config = Config::new();
    config.user("postgres");
    config.password("postgres");
    config.host("localhost");
    config.dbname("postgres_database");

    let (client, connection) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
