use actix_web::{web, App, HttpServer, http};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::ser::StdError;
use log::info;
use actix_cors::Cors;

#[macro_use]
extern crate diesel;
mod schema;

mod handlers;
mod models;
mod utils;

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
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")  // Frontend server address
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(pool.clone()) // Pass the database pool to the app
            // * statements
            .route("/statements", web::get().to(handlers::statement::get_statements_handler))
            .route("/statements/{slug}", web::get().to(handlers::statement::get_statement_handler))
            .route("/statements", web::post().to(handlers::statement::create_statement_handler))
            .route("/statements/{statement_id}", web::delete().to(handlers::statement::delete_statement_handler))
            .route("/statements/{statement_id}", web::put().to(handlers::statement::update_statement_handler))
            // * campaigns
            // * proposals
            // * organizations
            // * users
    })
    .bind("127.0.0.1:8081")
    .expect("Failed to bind to port 8081")
    .run()
    .await?;

    info!("\n\nServer is STOPING... at http://127.0.0.1:8081\n");
    Ok(())
}
