mod addr;
mod utils;
mod xrpbase58;
extern crate base58;
extern crate hex_literal;
extern crate rlp;
extern crate rustc_hex;
extern crate rustlibsecp256k1;
extern crate substrate_primitives;
extern crate tiny_keccak;
extern crate uuid;
use addr::*;
use bip39::{Language, Mnemonic, MnemonicType};
use rustc_hex::{FromHex, ToHex};
use rustlibsecp256k1::{PublicKey, SecretKey};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

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

#[derive(Serialize)]
pub struct AddressValue {
    pub data: HashMap<String, String>,
}

#[wasm_bindgen]
pub fn get_address(hex_num: &str) -> JsValue {
    let secret: Vec<u8> = hex_num.from_hex().unwrap();
    let seckey = SecretKey::parse_slice(&secret[..]).unwrap();
    let pubkey = PublicKey::from_secret_key(&seckey);
    let pser = pubkey.serialize_compressed();

    let addr_bitcoin = addr_bitcoin_fork(
        &pser,
        AddrNetwork::BitcoinMainnet,
        AddrHashKind::P2PKH,
        false,
    );

    let addr_bitcoin_test = addr_bitcoin_fork(
        &pser,
        AddrNetwork::BitcoinTestnet,
        AddrHashKind::P2PKH,
        false,
    );

    let addr_litecoin = addr_bitcoin_fork(
        &pser,
        AddrNetwork::LitecoinMainnet,
        AddrHashKind::P2PKH,
        false,
    );
    let addr_dogecoin = addr_bitcoin_fork(
        &pser,
        AddrNetwork::DogecoinMainnet,
        AddrHashKind::P2PKH,
        false,
    );

    let addr_ethereum = addr_ethereum_fork(&pubkey.serialize()[..], true);

    let addr_ripple = addr_ripple(&pser[..], false);

    let mut data = HashMap::new();
    data.insert(String::from("bitcoin"), addr_bitcoin);
    data.insert(String::from("bitcoin_test"), addr_bitcoin_test);
    data.insert(String::from("litecoin"), addr_litecoin);
    data.insert(String::from("dogecoin"), addr_dogecoin);
    data.insert(String::from("ethereum"), addr_ethereum);
    data.insert(String::from("ripple"), addr_ripple);
    let av = AddressValue { data };

    JsValue::from_serde(&av).unwrap()
}

#[wasm_bindgen]
pub fn bitcoin_addr(hex_num: &str) -> String {
    // let secret : Vec<u8> = "0000000000000000000000000000000000000000000000000000000000000001".from_hex().unwrap();
    let secret: Vec<u8> = hex_num.from_hex().unwrap();
    let seckey = SecretKey::parse_slice(&secret[..]).unwrap();
    let pubkey = PublicKey::from_secret_key(&seckey);
    addr_bitcoin_fork(
        &pubkey.serialize_compressed(),
        AddrNetwork::BitcoinMainnet,
        AddrHashKind::P2PKH,
        false,
    )
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
    // let pri_hex_string = to_hex_string(secret.to_vec());
    let pri_hex_string = secret.to_hex::<String>();

    // let hex_string = to_hex_string(pubkey.serialize_compressed().to_vec());
    let hex_string = pubkey.serialize_compressed().to_hex::<String>();
    console_log!("secp256k1 test");
    console_log!("private key: {}", pri_hex_string);
    console_log!("public key: {}", hex_string);

    let addr = addr_bitcoin_fork(
        &pubkey.serialize(),
        AddrNetwork::BitcoinMainnet,
        AddrHashKind::P2PKH,
        false,
    );
    let addr_compressed = addr_bitcoin_fork(
        &pubkey.serialize_compressed(),
        AddrNetwork::BitcoinMainnet,
        AddrHashKind::P2PKH,
        false,
    );
    console_log!("bitcoin address: {}", addr_compressed);
    console_log!("uncompressed address: {}", addr);

    // Bitcoin address: 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH
    // uncompressed:  1EHNa6Q4Jz2uvNExL497mE43ikXhwF6kZm
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Greet");
}

#[wasm_bindgen]
pub fn get_mnemonic() -> String {
    // create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    // get the phrase
    mnemonic.phrase().to_string()
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn log_hello_marco() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}
