#[macro_use]
extern crate failure;
mod database;
mod smt;
mod merkletrie_interface;
mod merkletrie;
mod hashtree;
use smt::{sparse_main, sparse_order};
use merkletrie::{patricia_main, patricia_order};
use hashtree::starling_main;
pub fn main() {
    //sparse_order();
    //patricia_order();
    sparse_main();
    patricia_main();   
    starling_main();
}
