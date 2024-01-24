use actix_web::{Responder, HttpResponse, web};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use crate::schema::statements::dsl::*;
// use crate::schema::statements::columns::*;
use crate::schema::statements::slug;
use crate::models::{Statement, NewStatement, UpdateStatement};
use crate::DbPool;
use log::{info, error};
use serde::{Serialize, Deserialize};

// * GET all statements
pub async fn get_statements_handler(db_pool: web::Data<DbPool>) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    
    info!("Attempting to fetch statements");
    match get_statements_query(&conn) {
        Ok(retrieved_statements) => {
            info!("Successfully fetched statements");
            HttpResponse::Ok().json(retrieved_statements)
        }
        Err(e) => {
            error!("Failed to fetch statements");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn get_statements_query(conn: &PgConnection) -> QueryResult<Vec<Statement>> {
    statements.load::<Statement>(conn)
}

// * GET one statement by content
pub async fn get_statement_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let stmt_slug = path.into_inner();
    // println!("{}", stmt_slug);
    info!("Attempting to fetch statement by slug");
    match get_statement_query(&conn, &stmt_slug) {
        Ok(statement) => {
            info!("Successfully fetched statement");
            HttpResponse::Ok().json(statement)
        }
        Err(e) => {
            error!("Failed to fetch statement");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn get_statement_query(conn: &PgConnection, stmt_slug: &str) -> QueryResult<Statement> {
    statements.filter(slug.eq(stmt_slug)).first(conn)
}

// * POST create statment

pub async fn create_statement_handler(
    db_pool: web::Data<DbPool>,
    new_stmt: web::Json<NewStatement>,
) -> impl Responder {
    let conn = db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    info!("Attempting to fetch statement by id");    
    match create_statement_query(&conn, new_stmt.into_inner()) {
        Ok(statement) => {
            info!("Successfully created statement");
            HttpResponse::Ok().json(statement)
        }, 
        Err(e) => {
            error!("Failed to create statement");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn create_statement_query(conn: &PgConnection, new_stmt: NewStatement) -> QueryResult<Statement> {
    diesel::insert_into(statements)
        .values(&new_stmt)
        .get_result::<Statement>(conn)
}

// * DELETE statements by id

#[derive(Serialize, Deserialize)]
pub struct StatementPath {
    statement_id: i32,
}

pub async fn delete_statement_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<StatementPath>,
) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let stmt_id = path.statement_id;

    info!("Attempting to delete statement with id");
    match delete_statement_query(&conn, stmt_id) {
        Ok(_) => {
            info!("Successfully deleted statement");
            HttpResponse::Ok().json("Statement deleted successfully")
        }
        Err(diesel::result::Error::NotFound) => {
            error!("Statement not found");
            HttpResponse::NotFound().json("Statement not found")
        }
        Err(e) => {
            error!("Failed to delete statement");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn delete_statement_query(conn: &PgConnection, stmt_id: i32) -> QueryResult<usize> {
   let num_deleted = diesel::delete(statements.find(stmt_id)).execute(conn)?;

    if num_deleted == 0 {
        Err(diesel::NotFound)
    } else {
        Ok(num_deleted)
    }
}

// * UPDATE (PUT) one statement by id
pub async fn update_statement_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<i32>,
    updated_statement_data: web::Json<UpdateStatement>,
) -> impl Responder {
    let stmt_id = path.into_inner();
    let conn = db_pool.get().expect("Failed to get DB connection");
    info!("Attempting to update statement with new data");

    match update_statement_query(&conn, stmt_id, updated_statement_data.into_inner()) {
        Ok(updated_statement) => {
            info!("Successfully updated statement");
            HttpResponse::Ok().json(updated_statement)
        }
        Err(e) => {
            error!("Failed to update statement");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn update_statement_query(
    conn: &PgConnection,
    stmt_id: i32,
    changeset: UpdateStatement,
) -> QueryResult<Statement> {
    diesel::update(statements.find(stmt_id))
        .set(&changeset)
        .get_result::<Statement>(conn)
}

// pub fn update_statement_query<ST2>(
//     conn: &PgConnection,
//     stmt_id: i32,
//     data: &UpdateStatement,
// ) -> QueryResult<()> {
//     let public_rating_str = data.public_rating.as_ref().map(|r| r.to_string());
//     let our_rating_str = data.our_rating.as_ref().map(|r| r.to_string());
//     let content_str = data.content.as_ref().map(|s| s.as_str());
//     let context_str = data.context.as_ref().map(|s| s.as_str());

//     diesel::sql_query("UPDATE statements SET public_rating = $1, our_rating = $2, content = $3, context = $4 WHERE statement_id = $5")
//         .bind::<Nullable<Text>, _>(public_rating_str)
//         .bind::<Nullable<Text>, _>(our_rating_str)
//         .bind::<Nullable<Text>, _>(content_str)
//         .bind::<Nullable<Text>, _>(context_str)
//         .bind::<ST2, i32>(stmt_id)
//         .execute(conn)?;

//     Ok(())
// }
