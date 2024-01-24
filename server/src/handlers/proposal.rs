use actix_web::{Responder, HttpResponse, web};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use crate::schema::proposals::dsl::*;
use crate::schema::proposals::columns::*;
use crate::models::{Proposal, NewProposal, UpdateProposal};
use crate::DbPool;
use log::{info, error};
use serde::{Serialize, Deserialize};

// * GET all proposals
pub async fn get_proposals_handler(db_pool: web::Data<DbPool>) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    
    info!("Attempting to fetch proposals");
    match get_proposals_query(&conn) {
        Ok(retrieved_proposals) => {
            info!("Successfully fetched proposals");
            HttpResponse::Ok().json(retrieved_proposals)
        }
        Err(e) => {
            error!("Failed to fetch proposals");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn get_proposals_query(conn: &PgConnection) -> QueryResult<Vec<Proposal>> {
    proposals.load::<Proposal>(conn)
}

// * GET one proposal by content
pub async fn get_proposal_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let stmt_slug = path.into_inner();
    // println!("{}", stmt_slug);
    info!("Attempting to fetch proposal by slug");
    match get_proposal_query(&conn, &stmt_slug) {
        Ok(proposal) => {
            info!("Successfully fetched proposal");
            HttpResponse::Ok().json(proposal)
        }
        Err(e) => {
            error!("Failed to fetch proposal");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn get_proposal_query(conn: &PgConnection, stmt_slug: &str) -> QueryResult<Proposal> {
    proposals.filter(slug.eq(stmt_slug)).first::<Proposal>(conn)
}

// * POST create proposals

pub async fn create_proposal_handler(
    db_pool: web::Data<DbPool>,
    new_prpsl: web::Json<NewProposal>,
) -> impl Responder {
    let conn = db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    info!("Attempting to fetch proposal by id");    
    match create_proposal_query(&conn, new_prpsl.into_inner()) {
        Ok(proposal) => {
            info!("Successfully created proposal");
            HttpResponse::Ok().json(proposal)
        }, 
        Err(e) => {
            error!("Failed to create proposal");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn create_proposal_query(conn: &PgConnection, new_prpsl: NewProposal) -> QueryResult<Proposal> {
    diesel::insert_into(proposals)
        .values(&new_prpsl)
        .get_result::<Proposal>(conn)
}

// * DELETE proposals by id

#[derive(Serialize, Deserialize)]
pub struct ProposalPath {
    proposal_id: i32,
}

pub async fn delete_proposal_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<ProposalPath>,
) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection");
    let stmt_id = path.proposal_id;

    info!("Attempting to delete proposal with id");
    match delete_proposal_query(&conn, stmt_id) {
        Ok(_) => {
            info!("Successfully deleted proposal");
            HttpResponse::Ok().json("Proposal deleted successfully")
        }
        Err(diesel::result::Error::NotFound) => {
            error!("Proposal not found");
            HttpResponse::NotFound().json("Proposal not found")
        }
        Err(e) => {
            error!("Failed to delete proposal");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn delete_proposal_query(conn: &PgConnection, stmt_id: i32) -> QueryResult<usize> {
   let num_deleted = diesel::delete(proposals.find(stmt_id)).execute(conn)?;

    if num_deleted == 0 {
        Err(diesel::NotFound)
    } else {
        Ok(num_deleted)
    }
}

// * UPDATE (PUT) one proposal by id
pub async fn update_proposal_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<i32>,
    updated_proposal_data: web::Json<UpdateProposal>,
) -> impl Responder {
    let stmt_id = path.into_inner();
    let conn = db_pool.get().expect("Failed to get DB connection");
    info!("Attempting to update proposal with new data");

    match update_proposal_query(&conn, stmt_id, updated_proposal_data.into_inner()) {
        Ok(updated_proposal) => {
            info!("Successfully updated proposal");
            HttpResponse::Ok().json(updated_proposal)
        }
        Err(e) => {
            error!("Failed to update proposal");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn update_proposal_query(
    conn: &PgConnection,
    stmt_id: i32,
    changeset: UpdateProposal,
) -> QueryResult<Proposal> {
    diesel::update(proposals.find(stmt_id))
        .set(&changeset)
        .get_result::<Proposal>(conn)
}
