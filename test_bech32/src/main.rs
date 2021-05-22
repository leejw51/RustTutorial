use bech32::{self, FromBase32, ToBase32, Variant};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use ethsign::SecretKey;
use std::convert::{TryFrom, TryInto};
//use tiny_hderive::bip32::ExtendedPrivKey;
use bitcoin::{
    network::constants::Network,
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
};
use hdpath::StandardHDPath;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};
use failure::format_err;

fn test_eth()-> Result<(), failure::Error> {
	let mnemonicwords=std::env::var("MYMNEMONICS")?;
    let mnemonic = 
    Mnemonic::from_phrase(mnemonicwords.as_str(), Language::English).map_err(|e| failure::format_err!("get mnemonics"))?;
    let seed = Seed::new(&mnemonic, "");

    let seed_bytes: &[u8] = seed.as_bytes();
    println!("seed bytes {:?}", seed_bytes);

    let ext =
        tiny_hderive::bip32::ExtendedPrivKey::derive(seed_bytes, "m/44'/394'/0'/0/0").map_err(|_| format_err!("tiny_hderive"))?;
    println!("extended privkey {:?}", ext);
    let secret_key = SecretKey::from_raw(&ext.secret())?;
    let public_key = secret_key.public();
    println!("secretkey {:?}", secret_key);
    println!("pubkey {:?}", public_key);
    println!("address {:?}", public_key.address());
    let encoded = bech32::encode("cro", public_key.address().to_base32(), Variant::Bech32).map_err(|_|format_err!("bech32"))?;
    println!("{:?}", public_key.address().len());
    println!("{}", encoded);
    Ok(())
}

/// Decode an extended private key from a mnemonic
fn private_key_from_mnemonic(
    mnemonic_words: &str,
    coin_type: u32,
) -> Result<ExtendedPrivKey, failure::Error> {
    let mnemonic = Mnemonic::from_phrase(mnemonic_words, Language::English).map_err(|e| format_err!("get menmonic"))?;

    let seed = Seed::new(&mnemonic, "");
    println!("seed bytes {:?}", seed.as_bytes());
    // Get Private Key from seed and standard derivation path
    let hd_path_format = format!("m/44'/{}'/0'/0/0", coin_type);
    println!("hd_path_format {}", hd_path_format);
    let hd_path = StandardHDPath::try_from(hd_path_format.as_str()).map_err(|_| format_err!("hdpath"))?;

    let private_key = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes())
        .and_then(|k| k.derive_priv(&Secp256k1::new(), &DerivationPath::from(hd_path)))
        .map_err(|_| format_err!("derivekey"))
        ?;
        
    println!("@@@@@@@@@@@@@@  ext priv key {:?}", private_key);

    Ok(private_key)
}

fn get_address(pk: ExtendedPubKey) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(pk.public_key.to_bytes().as_slice());

    // Read hash digest over the public key bytes & consume hasher
    let pk_hash = hasher.finalize();

    // Plug the hash result into the next crypto hash function.
    let mut rip_hasher = Ripemd160::new();
    rip_hasher.update(pk_hash);
    let rip_result = rip_hasher.finalize();

    rip_result.to_vec()
}

fn test_bitcoin() -> Result<(), failure::Error> {
    let mnemonic = std::env::var("MYMNEMONICS")?;

    let coin_type = 394;

    println!("mnemonics {}", mnemonic);
    println!("coin type {:?}", coin_type);
    // Get the private key from the mnemonic
    let private_key = private_key_from_mnemonic(mnemonic.as_str(), coin_type)?;
    println!("privatekey {:?}", private_key);

    // Get the public Key from the private key
    let public_key = ExtendedPubKey::from_private(&Secp256k1::new(), &private_key);
    println!("publickey {:?}", public_key);

    // Get address from the public Key
    let address = get_address(public_key);
    println!("addres {:?}", address);

    // Compute Bech32 account
    let account = bech32::encode("cro", address.to_base32(), Variant::Bech32).map_err(|e| format_err!("get account error"))?;
	println!("account {}", account);
    Ok(())
}

fn main() {
    test_bitcoin().unwrap();
}
