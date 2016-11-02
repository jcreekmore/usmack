#![deny(warnings)]
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

pub mod errors;
pub mod label;

pub use label::Label;
