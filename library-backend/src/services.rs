//! Service layer for the library application
//!
//! This module provides the business logic for managing books and users
//! in the library system.

pub mod book_service;
pub mod user_service;
pub mod borrow;
pub mod statistics_service;

// Re-export the main service structs for easier access
pub use book_service::BookService;
pub use user_service::UserService;
pub use borrow::BorrowService;