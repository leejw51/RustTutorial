use ethsign::SecretKey;
use std::str::FromStr;
use tiny_hderive::bip32::ExtendedPrivKey;
use tiny_hderive::bip44::ChildNumber;
fn main() {
    println!("Hello, world!");
    // Seed should be generated from your BIP39 phrase first!
    let seed: &[u8] = &[42; 64];
    for i in 0..5 {
        let account =
            ExtendedPrivKey::derive(seed, format!("m/44'/60'/0'/0/{}", i).as_str()).unwrap();
        let secret_key = account.secret();
        let secret_key2 = SecretKey::from_raw(&secret_key).unwrap();
        let public_key = secret_key2.public();
        println!(
            "({})  private_key={} length={} \npublic_key={}\naddress={}",
            i,
            hex::encode(secret_key),
            secret_key.len(),
            hex::encode(public_key.bytes().to_vec()),
            hex::encode(public_key.address().to_vec()),
        );
    }
}
