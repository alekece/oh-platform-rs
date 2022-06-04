#![allow(clippy::pedantic)]

#[macro_use]
extern crate diesel;

pub mod catchers;
mod database;
mod error;
mod response;
pub mod routes;
mod schema;

pub use database::Database;
pub use error::Error;
pub use response::Response;
