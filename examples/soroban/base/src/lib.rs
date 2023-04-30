#![no_std]
use loam_sdk::{soroban_contract, soroban_sdk};
use loam_sdk_core_riffs::{owner::Owner, Ownable};

pub struct Contract;

impl Ownable for Contract {
    type Impl = Owner;
}

soroban_contract!();
