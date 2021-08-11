#![no_main]
#![no_std]

extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, U256};
use contract_utils::contract_context;

#[no_mangle]
fn balance_of() {
    erc20::balance_of()
}

#[no_mangle]
fn transfer() {
    erc20::transfer()
}

#[no_mangle]
fn approve() {
    erc20::approve()
}

#[no_mangle]
fn allowance() {
    erc20::allowance()
}

#[no_mangle]
fn transfer_from() {
    erc20::transfer_from()
}

#[no_mangle]
fn total_supply() {
    erc20::total_supply()
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    erc20::init(name, symbol, decimals);
    erc20::mint(contract_context::get_caller(), initial_supply);
}

fn get_entry_points() -> casper_types::EntryPoints {
    use casper_types::CLTyped;
    let mut entry_points = casper_types::EntryPoints::new();
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "constructor",
        alloc::vec![
            casper_types::Parameter::new("name", <String>::cl_type()),
            casper_types::Parameter::new("symbol", <String>::cl_type()),
            casper_types::Parameter::new("decimals", <u8>::cl_type()),
            casper_types::Parameter::new("initial_supply", <U256>::cl_type()),
        ],
        <()>::cl_type(),
        casper_types::EntryPointAccess::Groups(alloc::vec![casper_types::Group::new(
            "constructor"
        )]),
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "transfer",
        alloc::vec![
            casper_types::Parameter::new("recipient", <Key>::cl_type()),
            casper_types::Parameter::new("amount", <U256>::cl_type()),
        ],
        <()>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "transfer_from",
        alloc::vec![
            casper_types::Parameter::new("owner", <Key>::cl_type()),
            casper_types::Parameter::new("recipient", <Key>::cl_type()),
            casper_types::Parameter::new("amount", <U256>::cl_type()),
        ],
        <()>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "approve",
        alloc::vec![
            casper_types::Parameter::new("spender", <Key>::cl_type()),
            casper_types::Parameter::new("amount", <U256>::cl_type()),
        ],
        <()>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "balance_of",
        alloc::vec![casper_types::Parameter::new("owner", <Key>::cl_type())],
        <U256>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "allowance",
        alloc::vec![
            casper_types::Parameter::new("owner", <Key>::cl_type()),
            casper_types::Parameter::new("spender", <Key>::cl_type()),
        ],
        <U256>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(casper_types::EntryPoint::new(
        "total_supply",
        ::alloc::vec::Vec::new(),
        <U256>::cl_type(),
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, v) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let name: String = casper_contract::contract_api::runtime::get_named_arg("name");
    let symbol: String = casper_contract::contract_api::runtime::get_named_arg("symbol");
    let decimals: u8 = casper_contract::contract_api::runtime::get_named_arg("decimals");
    let initial_supply: U256 =
        casper_contract::contract_api::runtime::get_named_arg("initial_supply");
    let mut constructor_args = casper_types::RuntimeArgs::new();
    constructor_args.insert("name", name).unwrap_or_revert();
    constructor_args.insert("symbol", symbol).unwrap_or_revert();
    constructor_args
        .insert("decimals", decimals)
        .unwrap_or_revert();
    constructor_args
        .insert("initial_supply", initial_supply)
        .unwrap_or_revert();
    let constructor_access: casper_types::URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);
    let mut urefs = alloc::collections::BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &alloc::format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &alloc::format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &alloc::format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &alloc::format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &alloc::format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
