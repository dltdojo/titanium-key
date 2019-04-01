mod utils;

extern crate bitcoin;
extern crate rustlibsecp256k1;

use bip39::{Language, Mnemonic, MnemonicType};
use rustlibsecp256k1::{PublicKey, SecretKey};
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
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn log_hello() {
    log("Hello from Rust!");
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

#[wasm_bindgen]
pub fn secp256k1_key() {
    //
    // gen privatekey and publickey from crate rustlibsecp256k1
    // https://docs.rs/crate/libsecp256k1/0.2.2/source/tests/verify.rs
    //
    let secret: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ];

    let seckey = SecretKey::parse(&secret).unwrap();
    let pubkey = PublicKey::from_secret_key(&seckey);
    let pubkey_compressed = PublicKey::parse_compressed(&pubkey.serialize_compressed()).unwrap();
    let pri_hex_string = to_hex_string(secret.to_vec());
    let hex_string = to_hex_string(pubkey_compressed.serialize().to_vec());
    console_log!("secp256k1 test");
    console_log!("private key: {}", pri_hex_string);
    console_log!("public key: {}", hex_string);

    //
    // gen bitcoin address from crate bitcoin
    // https://github.com/rust-bitcoin/rust-bitcoin/blob/cea49b6522abada5f34d1034804b89cf24998d61/src/util/address.rs
    // https://github.com/rust-bitcoin/rust-bitcoin/blob/cea49b6522abada5f34d1034804b89cf24998d61/src/util/key.rs
    //
    // let public_key = key::PublicKey::from_slice(&pubkey.serialize()[..]).unwrap();
    //  note: rust-lld: error: unknown file type: lax_der_parsing.o

    // Generate pay-to-pubkey-hash address
    // let address = Address::p2pkh(&public_key, Network::Bitcoin);
    // println!("{}", &address.to_string());
    // console_log!("address: {}", &address.to_string());
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("hello");
}

#[wasm_bindgen]
pub fn get_mnemonic() {
    // create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    // get the phrase
    let phrase: &str = mnemonic.phrase();
    console_log!("bip39 mnemonic phrase: {}", phrase);
}

#[wasm_bindgen(start)]
pub fn run() {
    log_hello();
    log_hello_marco();
}

fn log_hello_marco() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}
