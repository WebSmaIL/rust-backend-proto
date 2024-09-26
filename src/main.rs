// src/main.rs
mod models;
mod schema;

#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}