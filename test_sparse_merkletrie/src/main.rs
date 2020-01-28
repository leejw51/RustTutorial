#[macro_use]
extern crate failure;
mod database;
mod dynamic_smt;
mod hashtree;
mod merkletrie;
mod merkletrie_interface;
mod smt;
use bitvec::prelude::*;
use bitvec::*;
use hashtree::starling_main;
use merkletrie::{patricia_main, patricia_order};
use smt::{sparse_main, sparse_order};
fn test_order() {
    sparse_order();
    patricia_order();
}
fn benchmark() {
    sparse_main();
    patricia_main();
    starling_main();
}
fn binary_test() {
    println!("-----------");
    //big endian
    let mut bv = bitvec![Msb0, u8; 0,0,0, 1, 0, 1];
    for i in 0..512 {
        bv.push(true);
    }
    println!("0 flag={}", bv[3]);

    let m = bincode::serialize(&bv).unwrap();
    let bv2: BitVec<Msb0, u8> = bincode::deserialize(&m).unwrap();
    let m2 = bincode::serialize(&bv2).unwrap();
    assert!(m == m2);

    println!("{:?}", bv);
    println!("{:?}", &bv[0..3]);
    println!("encoded:{} bytes", m.len());
}
pub fn main() {
    //binary_test();
    dynamic_smt::dynamic_sparse_main();
}
