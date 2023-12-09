use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::ser::StdError;
use log::info;

#[macro_use]
extern crate diesel;
mod schema;

mod handlers;
mod models;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// The main function, the entry point of the application
#[actix_web::main]
async fn main() -> Result<(), Box<(dyn StdError + 'static)>> {
    dotenv().ok(); // Load environment variables
    env_logger::init();

    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    info!("\n\nServer is STARTING... at http://127.0.0.1:8081\n");

    // Set up HTTP server with route configuration
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Pass the database pool to the app
            // Define routes
            .route("/statements", web::get().to(handlers::statement::get_statements_handler))
            .route("/statements", web::post().to(handlers::statement::create_statement_handler))
    })
    .bind("127.0.0.1:8081")
    .expect("Failed to bind to port 8081")
    .run()
    .await?;

    info!("\n\nServer is STOPING... at http://127.0.0.1:8081\n");
    Ok(())
}
