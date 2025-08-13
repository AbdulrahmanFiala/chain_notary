// ============================================================================
// QUERY MODULE COORDINATOR
// ============================================================================
// This module coordinates and re-exports all query functions organized by domain

// Re-export all query functions from their respective modules
pub use super::document_queries::*;
pub use super::collection_queries::*;
pub use super::institution_queries::*;
pub use super::search_queries::*;

