#![no_std]

extern crate alloc;

pub mod admin_control;
pub mod contract_context;
mod data;

pub use data::{get_key, key_to_str, set_key, Dict};
