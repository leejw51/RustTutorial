#[macro_use]
extern crate failure;
mod database;
mod smt;
mod dynamic_smt;
mod merkletrie_interface;
mod merkletrie;
mod hashtree;
use smt::{sparse_main, sparse_order};
use merkletrie::{patricia_main, patricia_order};
use hashtree::starling_main;

fn test_order() {
    sparse_order();
    patricia_order();
}
fn benchmark() {
    sparse_main();
    patricia_main();   
    starling_main();
}
pub fn main() {
    dynamic_smt::dynamic_sparse_main();
}
