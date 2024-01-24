// Declare the statement module
pub mod statement;

// Re-export the functions from the statement module
pub use statement::{
    get_statements_handler,
    get_statement_handler,
    create_statement_handler,
    delete_statement_handler,
    update_statement_handler
};

// Declare the proposal module
pub mod proposal;

pub use proposal::{
    get_proposals_handler,
    get_proposal_handler,
    create_proposal_handler,
    delete_proposal_handler,
    update_proposal_handler
};

// ... other handler modules