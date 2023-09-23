#![allow(unused)]
#![no_std]

use soroban_sdk::*;
use risc0_zkvm::Receipt;

#[contract]
struct Verifier;

#[contractimpl]
impl Verifier {
    pub fn verify(
        env: Env,
        receipt: ReceiptBundle,
    ) -> bool {
        todo!()
    }
}

#[contracttype]
pub struct ReceiptBundle {
    pub receipt: SuccinctReceipt,
    pub journal: Bytes,
}

#[contracttype]
pub struct SuccinctReceipt {
    pub seal: Vec<u32>,
    pub control_id: BytesN<32>,
    pub meta: Vec<u32>,
}
