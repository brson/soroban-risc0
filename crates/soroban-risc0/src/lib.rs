#![allow(unused)]
#![no_std]

use soroban_sdk::*;
use risc0_zkvm::Receipt;

extern crate alloc;
use alloc::vec::Vec as RustVec;

#[contract]
struct Verifier;

#[contractimpl]
impl Verifier {
    pub fn verify(
        env: Env,
        image_id: BytesN<32>,
        receipt: ReceiptBundle,
    ) -> bool {
        let receipt = receipt.to_receipt();
        //let res = receipt.verify(&image_id);

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

impl ReceiptBundle {
    fn to_receipt(&self) -> Receipt {
        todo!()
    }
}
