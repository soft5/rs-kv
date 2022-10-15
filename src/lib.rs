// This is to use executable files in bin/
// Should refactor the app to use this more?

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate lazy_static;

use std::io::stdin;

pub mod models;
pub mod security;

pub mod db;
pub mod pb;
pub use pb::abi::*;
mod error;
pub use error::KvError;
mod service;
pub use service::*;

mod storage;
pub use storage::*;
pub mod utils;

pub fn from_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input[..(input.len() - 1)].to_string();

    input
}
