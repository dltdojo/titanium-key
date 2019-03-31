mod utils;

extern crate rustlibsecp256k1;
extern crate bitcoin;

use rustlibsecp256k1::{PublicKey, SecretKey};
use bip39::{Mnemonic, MnemonicType, Language};
use wasm_bindgen::prelude::*;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn secp256k1_key() {
    // 
    // gen privatekey and publickey from crate rustlibsecp256k1
    // https://docs.rs/crate/libsecp256k1/0.2.2/source/tests/verify.rs
    // 
    let secret: [u8; 32] = [
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x01,
    ];

    let seckey = SecretKey::parse(&secret).unwrap();
    let pubkey = PublicKey::from_secret_key(&seckey);
    let pubkey_compressed = PublicKey::parse_compressed(&pubkey.serialize_compressed()).unwrap();

    //
    // gen bitcoin address from crate bitcoin
    // https://github.com/rust-bitcoin/rust-bitcoin/blob/cea49b6522abada5f34d1034804b89cf24998d61/src/util/address.rs
    // https://github.com/rust-bitcoin/rust-bitcoin/blob/cea49b6522abada5f34d1034804b89cf24998d61/src/util/key.rs
    //
    let public_key = key::PublicKey::from_slice(&pubkey.serialize()[..]).unwrap();

    // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
}

#[wasm_bindgen]
pub fn greet() {
   
    // create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    // get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);

    alert(phrase);
}
