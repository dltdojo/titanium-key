mod utils;
mod addr;
extern crate base58;
extern crate rustlibsecp256k1;
use bip39::{Language, Mnemonic, MnemonicType};
use rustlibsecp256k1::{PublicKey, SecretKey};
use wasm_bindgen::prelude::*;
use addr::bitcoin_addr;

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
    // Add no-std support by elichai · Pull Request #100 · rust-bitcoin/rust-secp256k1 https://github.com/rust-bitcoin/rust-secp256k1/pull/100
    // Allow to use external default callbacks by real-or-random · Pull Request #595 · bitcoin-core/secp256k1 https://github.com/bitcoin-core/secp256k1/pull/595
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
    // let pubkey_compressed = PublicKey::parse_compressed(&pubkey.serialize_compressed()).unwrap();
    let pri_hex_string = to_hex_string(secret.to_vec());
    let hex_string = to_hex_string(pubkey.serialize_compressed().to_vec());
    console_log!("secp256k1 test");
    console_log!("private key: {}", pri_hex_string);
    console_log!("public key: {}", hex_string);

	let addr = bitcoin_addr(&pubkey.serialize());
	let addr_compressed = bitcoin_addr(&pubkey.serialize_compressed());
    console_log!("bitcoin address: {}", addr_compressed);
    console_log!("uncompressed address: {}", addr);

    // Bitcoin address: 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH
    // uncompressed:  1EHNa6Q4Jz2uvNExL497mE43ikXhwF6kZm
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
