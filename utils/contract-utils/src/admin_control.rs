use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, Key};

use crate::{contract_context, Dict};

const ADMINS_DICT: &str = "admins";

pub fn init() {
    Admins::init();
}

pub fn add_admin(address: Key) {
    assert_caller_is_admin();
    add_admin_without_check(address);
}

pub fn disable_admin(address: Key) {
    assert_caller_is_admin();
    Admins::instance().disable_admin(&address);
}

pub fn add_admin_without_check(address: Key) {
    Admins::instance().add_admin(&address);
}

fn assert_caller_is_admin() {
    let caller = contract_context::get_caller();
    if !Admins::instance().is_admin(&caller) {
        runtime::revert(ApiError::User(20));
    }
}

struct Admins {
    dict: Dict,
}

impl Admins {
    pub fn instance() -> Admins {
        Admins {
            dict: Dict::instance(ADMINS_DICT),
        }
    }

    pub fn init() {
        storage::new_dictionary(ADMINS_DICT).unwrap_or_revert();
    }

    pub fn is_admin(&self, key: &Key) -> bool {
        self.dict.get_by_key::<()>(key).is_some()
    }

    pub fn add_admin(&self, key: &Key) {
        self.dict.set_by_key(key, ());
    }

    pub fn disable_admin(&self, key: &Key) {
        self.dict.remove_by_key::<()>(key);
    }
}
