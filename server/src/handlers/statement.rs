use actix_web::{Responder, HttpResponse, web};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::statements::dsl::*;
use crate::models::{Statement, NewStatement};
use crate::DbPool;
use log::{debug, info};

pub async fn get_statements_handler(db_pool: web::Data<DbPool>) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    
    info!("Attempting to fetch statements");
    match get_statements_query(&conn) {
        Ok(retrieved_statements) => {
            info!("Successfully fetched statements");
            HttpResponse::Ok().json(retrieved_statements)
        }
        Err(e) => {
            debug!("Failed to fetch statements: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn get_statements_query(conn: &PgConnection) -> QueryResult<Vec<Statement>> {
    statements.load::<Statement>(conn)
}

pub async fn create_statement_handler(
    db_pool: web::Data<DbPool>,
    new_stmt: web::Json<NewStatement>,
) -> impl Responder {
    let conn = db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    match create_statement_query(&conn, new_stmt.into_inner()) {
        Ok(statement) => HttpResponse::Ok().json(statement),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn create_statement_query(conn: &PgConnection, new_stmt: NewStatement) -> QueryResult<Statement> {
    diesel::insert_into(statements)
        .values(&new_stmt)
        .get_result(conn)
}
