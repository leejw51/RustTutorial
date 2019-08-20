use bip39::{Language, Mnemonic, Seed};
use ethsign::SecretKey;
use std::str::FromStr;
use tiny_hderive::bip32::ExtendedPrivKey;
use tiny_hderive::bip44::ChildNumber;
fn main() {
    let phrase = "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside";
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");

    println!("mnemonic={}", mnemonic);
    println!(
        "seed {} length={}",
        hex::encode(seed.as_bytes()),
        seed.as_bytes().len()
    );
    // Seed should be generated from your BIP39 phrase first!
    //let seed: &[u8] = &[42; 64];
    for i in 0..5 {
        let account =
            ExtendedPrivKey::derive(seed.as_bytes(), format!("m/44'/60'/0'/0/{}", i).as_str())
                .unwrap();
        let secret_key = account.secret();
        let secret_key2 = SecretKey::from_raw(&secret_key).unwrap();
        let public_key = secret_key2.public();
        println!(
            "({})  private_key={} length={} \npublic_key={} length={}\naddress={}  length={}",
            i,
            hex::encode(secret_key),
            secret_key.len(),
            hex::encode(public_key.bytes().to_vec()),
            public_key.bytes().len(),
            hex::encode(public_key.address().to_vec()),
            public_key.address().len(),
        );
    }
}
