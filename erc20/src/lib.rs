#![no_std]

extern crate alloc;

mod data;
mod erc20;

pub use erc20::{
    allowance, approve, balance_of, init, mint, total_supply, transfer, transfer_from,
};
