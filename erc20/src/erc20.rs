use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{CLValue, Key, U256};
use contract_utils::contract_context;

use crate::data::{self, Allowances, Balances};

pub fn init(name: String, symbol: String, decimals: u8) {
    data::set_name(name);
    data::set_symbol(symbol);
    data::set_decimals(decimals);
    Balances::init();
    Allowances::init();
}

pub fn mint(recipient: Key, amount: U256) {
    let balances = Balances::instance();
    let balance = balances.get(&recipient);
    balances.set(&recipient, balance + amount);

    data::set_total_supply(data::total_supply() + amount);
}

pub fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let balance = Balances::instance().get(&owner);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

pub fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    make_transfer(contract_context::get_caller(), recipient, amount);
}

pub fn approve() {
    let spender: Key = casper_contract::contract_api::runtime::get_named_arg("spender");
    let amount: U256 = casper_contract::contract_api::runtime::get_named_arg("amount");
    Allowances::instance().set(&contract_context::get_caller(), &spender, amount);
}

pub fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let allowance = Allowances::instance().get(&owner, &spender);
    runtime::ret(CLValue::from_t(allowance).unwrap_or_revert());
}

pub fn transfer_from() {
    let owner: Key = casper_contract::contract_api::runtime::get_named_arg("owner");
    let recipient: Key = casper_contract::contract_api::runtime::get_named_arg("recipient");
    let amount: U256 = casper_contract::contract_api::runtime::get_named_arg("amount");

    let allowances = Allowances::instance();
    let spender = contract_context::get_caller();
    let spender_allowance = allowances.get(&owner, &spender);
    allowances.set(&owner, &spender, spender_allowance - amount);
    make_transfer(owner, recipient, amount);
}

pub fn total_supply() {
    let total_supply = data::total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

fn make_transfer(sender: Key, recipient: Key, amount: U256) {
    let balances = Balances::instance();

    let sender_balance = balances.get(&sender);
    let recipient_balance = balances.get(&recipient);

    balances.set(&sender, sender_balance - amount);
    balances.set(&recipient, recipient_balance + amount);
}
