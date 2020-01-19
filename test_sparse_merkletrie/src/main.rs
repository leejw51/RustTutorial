#[macro_use]
extern crate failure;
mod database;
mod smt;
use smt::sparse_main;
pub fn main() {
    sparse_main();
}
