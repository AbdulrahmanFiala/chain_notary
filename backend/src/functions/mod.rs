pub mod document;
pub mod institution;
pub mod analytics;
pub mod user_management;

// Query submodules
pub mod document_queries;
pub mod institution_queries;
pub mod search_queries;
pub mod admin_queries;

pub use document::*;
pub use institution::*;
pub use analytics::*;
pub use user_management::*;
pub use admin_queries::*; 