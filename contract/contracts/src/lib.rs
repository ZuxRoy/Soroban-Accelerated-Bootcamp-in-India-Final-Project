#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, String, Env};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Files {
    pub name: String,
    pub description: String,
    pub details: String,
}

#[contract]
pub struct FileShare;

#[contractimpl]
impl FileShare {
    pub fn share_files(env: Env, user: Address, name: String, description: String, details: String) {
        let files = Files {
            name,
            description,
            details,
        };
        env.storage().persistent().set(&user, &files);
    }

    pub fn get_files(env: Env, user: Address) -> Option<Files> {
        env.storage().persistent().get::<Address, Files>(&user)
    }
}

mod test;
