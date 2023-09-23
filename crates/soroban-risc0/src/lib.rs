#![no_std]

use soroban_sdk::*;

#[contract]
struct Verifier;

#[contractimpl]
impl Verifier {
    pub fn verify(
        env: Env,
        bundle: Bytes,
    ) -> bool {
        todo!()
    }
}
