#[macro_use]
extern crate failure;
mod database;
mod smt;
mod merkletrie_interface;
mod merkletrie;
use smt::sparse_main;
use merkletrie::patricia_main;
pub fn main() {
    sparse_main();
    patricia_main();
}
