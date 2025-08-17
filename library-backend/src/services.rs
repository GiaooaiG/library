//! Service layer for the library application
//!
//! This module provides the business logic for managing books and users
//! in the library system.

pub mod book;
pub mod user;
pub mod borrow;
pub mod statistics;

// Re-export the main service structs for easier access
pub use book::BookService;
pub use user::UserService;
pub use borrow::BorrowService;
pub use statistics::StatisticsService;