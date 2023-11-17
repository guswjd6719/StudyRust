//use actix_web::{App, HttpServer};




use sea_orm::Database;
use sea_orm::ConnectOptions;
use std::time::Duration;

use sea_orm::entity::prelude::*;

#[actix_web::main]
async fn main() {
    dbConn().await;
}

async fn dbConn() -> Result<(), Box<dyn std::error::Error>>{
    let mut opt = ConnectOptions::new("mysql://seaorm:password@127.0.0.1:3306/bakery");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true) 
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;

    println!("{db:?}\n");

    println!("===== =====\n");
    let cheese: Option<cake::Model> = Cake::find_by_id(1).one(db).await?;

    println!();

    Ok(())
}
