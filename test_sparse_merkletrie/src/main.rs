#[macro_use]
extern crate failure;
mod database;
mod smt;
mod merkletrie_interface;
mod merkletrie;
mod hashtree;
use smt::sparse_main;
use merkletrie::patricia_main;
use hashtree::starling_main;
pub fn main() {
   sparse_main();
   patricia_main();
   starling_main();
}
