#[macro_use]
extern crate failure;
mod database;
mod dynamic_smt;
mod hashtree;
mod merkletrie;
mod merkletrie_interface;
mod smt;
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
pub fn main() {
    dynamic_smt::dynamic_sparse_main();
}
