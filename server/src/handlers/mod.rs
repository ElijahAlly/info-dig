// Declare the statement module
pub mod statement;

// Re-export the functions from the statement module
pub use statement::{get_statements_handler, create_statement_handler};

// ... other handler modules