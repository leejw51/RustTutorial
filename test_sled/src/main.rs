mod test;
mod sled_test;
fn main() {
    let count=1000;
    sled_test::write_db(count).unwrap();
    sled_test::write_db_batch(count).unwrap();
}
