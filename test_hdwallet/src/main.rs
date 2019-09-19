use bip39::{Language, Mnemonic, Seed};

use std::str::FromStr;
use tiny_hderive::bip32::ExtendedPrivKey;
use tiny_hderive::bip44::ChildNumber;

use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

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
        let secret_key_bytes = account.secret();
        let secp = Secp256k1::new();
        let secret_key =
            SecretKey::from_slice(&secret_key_bytes).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        let sig = secp.sign(&message, &secret_key);
        assert!(secp.verify(&message, &sig, &public_key).is_ok());

        println!(
            "({})  private_key={}/{} public_key={}/{}",
            i,
            &secret_key.to_string()[..8],
            secret_key.len(),
            &public_key.to_string()[..8],
            public_key.to_string().len() / 2,
        );
    }
}
