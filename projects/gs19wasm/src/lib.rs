mod addr;
mod utils;
mod xrpbase58;
extern crate base58;
extern crate hex_literal;
extern crate rlp;
extern crate rustc_hex;
extern crate rustlibsecp256k1;
extern crate tiny_keccak;
extern crate uuid;
extern crate curve25519_dalek;
extern crate ed25519_dalek;
use addr::*;
use bip39::{Language, Mnemonic, MnemonicType};
use rustc_hex::{FromHex, ToHex};
use rustlibsecp256k1::PublicKey as Secp256k1PublicKey;
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
    let pubkey : Secp256k1PublicKey = secp256k1_to_pubkey(hex_num.to_string());
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
    let pubkey : Secp256k1PublicKey = secp256k1_to_pubkey(hex_num.to_string());
    addr_bitcoin_fork(
        &pubkey.serialize_compressed(),
        AddrNetwork::BitcoinMainnet,
        AddrHashKind::P2PKH,
        false,
    )
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
