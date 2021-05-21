use bech32::{self, FromBase32, ToBase32, Variant};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use ethsign::SecretKey;
use tiny_hderive::bip32::ExtendedPrivKey;

fn main() {
    //let mnemonic = Mnemonic::from_phrase("", Language::English).unwrap();
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let seed = Seed::new(&mnemonic, "");

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    let ext = ExtendedPrivKey::derive(seed_bytes, "m/44'/60'/0'/0/0").unwrap();
    let secret_key = SecretKey::from_raw(&ext.secret()).unwrap();
    let public_key = secret_key.public();
    let encoded = bech32::encode("eth", public_key.address().to_base32(), Variant::Bech32).unwrap();
    println!("{:?}", public_key.address().len());
    println!("{}", encoded);
}
