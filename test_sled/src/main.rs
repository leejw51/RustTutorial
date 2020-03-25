mod test;
mod sled_test;
mod rocks;
fn main() {
    let count=1000000;
    println!("writing {} items", count);
    print!("sled normal '");
    sled_test::write_db(count).unwrap();
    print!("sled batch '");
    sled_test::write_db_batch(count).unwrap();
    print!("rocks normal '");
    rocks::write_db(count).unwrap();
    print!("rocks batch '");
    rocks::write_db_batch(count).unwrap();
}

