#![deny(warnings)]
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

pub mod access_vector;
pub mod errors;
pub mod label;

pub use label::Label;
