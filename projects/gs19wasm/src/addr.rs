use base58::ToBase58;
use crate::utils::*;
use crate::xrpbase58::ToXrpBase58;
// use crate::utils::{to_hex_string};
use hex_literal::*;
use ripemd160::{Digest, Ripemd160};
use rustc_hex::{FromHex, ToHex};
use sha2::Sha256;
use tiny_keccak::Keccak;
use uuid::Uuid;
use rustlibsecp256k1::PublicKey as Secp256k1PublicKey;
use rustlibsecp256k1::SecretKey as Secp256k1SecretKey;
use ed25519_dalek::PublicKey as Ed25519PublicKey;
use ed25519_dalek::SecretKey as Ed25519SecretKey;

pub struct UuidCard {
    pub id: Uuid,
    pub name: String,
}

// https://github.com/paritytech/parity-bitcoin/blob/master/keys/src/network.rs
pub enum AddrNetwork {
    BitcoinMainnet,
    BitcoinTestnet,
    LitecoinMainnet,
    LitecoinTestnet,
    DogecoinMainnet,
    Ethereum,
}

// https://github.com/paritytech/parity-bitcoin/blob/master/keys/src/address.rs
pub enum AddrHashKind {
    /// Pay to PubKey Hash
    /// Common P2PKH which begin with the number 1, eg: 1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2.
    /// https://bitcoin.org/en/glossary/p2pkh-address
    P2PKH,
    /// Pay to Script Hash
    /// Newer P2SH type starting with the number 3, eg: 3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy.
    /// https://bitcoin.org/en/glossary/p2sh-address
    P2SH,
}

//
// https://github.com/DR-BoneZ/borker-rs/blob/95e9d9d9b9426fdcdbecd1050f96f69def708dc6/src/wallet/child.rs
// Address - Bitcoin Wiki https://en.bitcoin.it/wiki/Address
pub fn hash160(data: &[u8]) -> Vec<u8> {
    // https://docs.rs/sha2/0.8.0/sha2/index.html
    let mut hasher = Sha256::new();
    hasher.input(data);
    let sha_bytes = hasher.result();
    let mut hasher = Ripemd160::new();
    hasher.input(&sha_bytes);
    let result_bytes = hasher.result();
    result_bytes.to_vec()
}

pub fn sha256sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let sha_bytes = hasher.result();
    let mut hasher = Sha256::new();
    hasher.input(&sha_bytes);
    let result_bytes = hasher.result();
    result_bytes.to_vec()
}

pub fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut keccak = Keccak::new_keccak256();
    let mut result = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut result);
    result.to_vec()
}

pub fn addr_bitcoin_fork(
    pubkey_or_hash: &[u8],
    network: AddrNetwork,
    kind: AddrHashKind,
    is_key_hash: bool,
) -> String {
    // Base58 and Base58Check Encoding
    // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch04.asciidoc#base58-and-base58check-encoding

    // let base58check_version_prefix: u8 = 0x00;

    let base58check_version_prefix: u8 = match (network, kind) {
        (AddrNetwork::BitcoinMainnet, AddrHashKind::P2PKH) => 0x00,
        (AddrNetwork::BitcoinMainnet, AddrHashKind::P2SH) => 0x05,
        (AddrNetwork::BitcoinTestnet, AddrHashKind::P2PKH) => 0x6F,
        (AddrNetwork::BitcoinTestnet, AddrHashKind::P2SH) => 196,
        (AddrNetwork::LitecoinMainnet, AddrHashKind::P2PKH) => 0x30,
        (AddrNetwork::LitecoinMainnet, AddrHashKind::P2SH) => 0x32,
        (AddrNetwork::LitecoinTestnet, AddrHashKind::P2PKH) => 0x6F,
        (AddrNetwork::LitecoinTestnet, AddrHashKind::P2SH) => 0x3A,
        (AddrNetwork::DogecoinMainnet, AddrHashKind::P2PKH) => 0x1E,
        _ => 0x00,
    };

    // Base58Check version prefix
    let mut addr_bytes: Vec<u8> = vec![base58check_version_prefix];

    // check public key hash
    let payload: Vec<u8> = if is_key_hash {
        pubkey_or_hash.to_vec()
    } else {
        hash160(pubkey_or_hash)
    };
    addr_bytes.extend(payload);

    // checksum = SHA256(SHA256(prefix+data))
    let chksum = sha256sha256(&addr_bytes[..]);
    addr_bytes.extend(&chksum[0..4]);
    ToBase58::to_base58(addr_bytes.as_slice())
}

// ethereumbook/04keys-addresses.asciidoc at develop · ethereumbook/ethereumbook
// https://github.com/ethereumbook/ethereumbook/blob/develop/04keys-addresses.asciidoc
// Ethereum addresses are hexadecimal numbers, identifiers derived from the last 20 bytes of the Keccak-256 hash of the public key.
// https://github.com/oraclize/ethereum-keys-sgx/blob/master/src/keccak.rs
pub fn addr_ethereum_fork(pubkey: &[u8], is_eip55: bool) -> String {
    // It is worth noting that the public key is not formatted with the prefix (hex) 04 when the address is calculated.
    let hashed_key = keccak256(&pubkey[1..65]);
    let addr: String = hashed_key[12..].to_hex();
    if is_eip55 {
        eth_checksum(&addr)
    } else {
        addr
    }
}

// https://github.com/Ethereum/EIPs/blob/master/EIPS/eip-55.md
pub fn eth_checksum(addr: &str) -> String {
    // Remove "0x" prefix if exists and make everything lowercase
    let lc_addr = strip_0x(addr).to_lowercase();

    // keccak256 of address
    let hashed_address = keccak256(&lc_addr.as_bytes());
    let haddr = hashed_address[..].to_hex::<String>();

    // Print final checksum
    eip55_checksum(&lc_addr, &haddr)
}

// Remove "0x" prefix
fn strip_0x(addr: &str) -> &str {
    if &addr[0..2] == "0x" {
        &addr[2..]
    } else {
        addr
    }
}

// https://github.com/rrybarczyk/eth-checksum/blob/d5edec9a98962afb213e453da276c8de6d6b3ef0/src/main.rs
fn eip55_checksum(addr: &str, addr_hash: &str) -> String {
    // Define new empty string to hold checksum addr
    let mut checksum_addr = String::new();

    for (c, hash_char) in addr.chars().zip(addr_hash.chars()) {
        // If ith hash char is greater than 8, capitilize ith addr char
        if hash_char.to_digit(15) > Some(8) {
            checksum_addr.extend(c.to_uppercase());
        } else {
            checksum_addr.push(c);
        }
    }
    checksum_addr
}

pub fn addr_ripple(pubkey_or_hash: &[u8], is_key_hash: bool) -> String {
    // XRP Ledger addresses are encoded using base58  with the Ripple dictionary: rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz.
    // https://github.com/ripple/ripple-dev-portal/blob/master/content/_code-samples/address_encoding/encode_address.js#L4

    // Base58Check version prefix
    let mut addr_bytes: Vec<u8> = vec![0x00];

    // check public key hash
    let payload: Vec<u8> = if is_key_hash {
        pubkey_or_hash.to_vec()
    } else {
        hash160(pubkey_or_hash)
    };
    addr_bytes.extend(payload);

    // checksum = SHA256(SHA256(prefix+data))
    let chksum = sha256sha256(&addr_bytes[..]);
    addr_bytes.extend(&chksum[0..4]);
    ToXrpBase58::to_base58(addr_bytes.as_slice())
}

// TODO
pub fn addr_vanity() -> String {
    // https://github.com/samr7/vanitygen
    // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch04.asciidoc#vanity-addresses

    "TODO".to_string()
}

// TODO
// Proof of Burn
// 1CounterpartyXXXXXXXXXXXXXXXUWLpVr
// https://en.wikipedia.org/wiki/Counterparty_(platform)
//
pub fn addr_burn() -> String {
    // https://github.com/dltdojo/book2018-quantumxie/blob/master/projects/gij2018/GIJ2018.md
    // getBtcBurnAddress('1Ghost2o18o715','Z').toString()
    // 1Ghost2o18o715ZZZZZZZZZZZZZZfTUTEh
    // prefix = 1Ghost2o18o715
    // padded = 1Ghost2o18o715ZZZZZZZZZZZZZZZZZZZZ
    // hash160 = Base58.decode(padded).slice(1, 21);
    // address = new bitcorelib.Address(hash160, net);
    //          1Ghost2o18o715ZZZZZZZZZZZZZZfTUTEh

    "TODO".to_string()
}

// bitcoin script
// https://github.com/wildcatgerry/bitcoin-asm/blob/master/src/lib.rs
pub fn bitcoin_script_to_hex(cmdline: String) -> String {
    let script =
        "OP_DUP OP_HASH160 788464014149d93b4a6135f3d665a0a2d743e6c3 OP_EQUALVERIFY OP_CHECKSIG";

    let mut hex_buffer = String::new();
    for token in cmdline.split_whitespace() {
        if token.starts_with("OP_") {
            hex_buffer.push_str(&format!("{:X}", bitcoin_to_opcode(token)));
        } else {
            hex_buffer.push_str(&token.to_uppercase());
        }
    }

    hex_buffer
}

// https://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed
pub fn addr_eth_contract(caller_addr: Vec<u8>, nonce: Vec<u8>) -> String {
    let mut rlp = rlp::RlpStream::new_list(2);
    rlp.append(&caller_addr);
    rlp.append(&nonce);
    let hashed_rlp_data = keccak256(&rlp.out());
    hashed_rlp_data[12..].to_hex()
}

pub fn uuid_card(name: &str) -> UuidCard {
    let id = Uuid::new_v4();
    UuidCard {
        id: id.clone(),
        name: name.into(),
    }
}

// Add no-std support by elichai · Pull Request #100 · rust-bitcoin/rust-secp256k1 https://github.com/rust-bitcoin/rust-secp256k1/pull/100
// Allow to use external default callbacks by real-or-random · Pull Request #595 · bitcoin-core/secp256k1 https://github.com/bitcoin-core/secp256k1/pull/595
// gen privatekey and publickey from crate rustlibsecp256k1
// https://docs.rs/crate/libsecp256k1/0.2.2/source/tests/verify.rs
pub fn secp256k1_to_pubkey(pri_key_hex : String) -> Secp256k1PublicKey {
    let secret: Vec<u8> = pri_key_hex.from_hex().unwrap();
    let seckey = Secp256k1SecretKey::parse_slice(&secret[..]).unwrap();
    Secp256k1PublicKey::from_secret_key(&seckey)
}

// https://docs.rs/crate/ed25519-dalek/1.0.0-pre.1/source/tests/ed25519.rs
pub fn ed25519_to_pubkey(pri_key_hex : String) -> Ed25519PublicKey {
    let secret: Vec<u8> = pri_key_hex.from_hex().unwrap();
    let seckey = Ed25519SecretKey::from_bytes(&secret[..]).unwrap();
    (&seckey).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::{H256, U256};
    //use substrate_primitives::ed25519::Pair;

    #[test]
    fn test_hash160() {
        // https://github.com/bitpay/bitcore-lib/blob/master/test/crypto/hash.js#L80
        // var buf = new Buffer([0, 1, 2, 3, 253, 254, 255]);
        // hash.toString('hex').should.equal('7322e2bd8535e476c092934e16a6169ca9b707ec');
        let pubkey_hex = "00010203fdfeff";
        let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
        let x = hash160(&pubkey_vec[..]);
        assert_eq!(
            "7322e2bd8535e476c092934e16a6169ca9b707ec",
            x.to_hex::<String>()
        );
    }

    #[test]
    fn test_sha256sha256() {
        // https://github.com/bitpay/bitcore-lib/blob/master/test/crypto/hash.js#L80
        // var buf = new Buffer([0, 1, 2, 3, 253, 254, 255]);
        // hash.toString('hex').should.equal('7322e2bd8535e476c092934e16a6169ca9b707ec');
        let pubkey_hex = "00010203fdfeff";
        let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
        let x = sha256sha256(&pubkey_vec[..]);
        assert_eq!(
            "be586c8b20dee549bdd66018c7a79e2b67bb88b7c7d428fa4c970976d2bec5ba",
            x.to_hex::<String>()
        );
    }

    #[test]
    fn test_bitcoin_addr1() {
        // https://github.com/bitpay/bitcore-lib/blob/master/test/address.js#L338
        // var pubkey = new PublicKey('0285e9737a74c30a873f74df05124f2aa6f53042c2fc0a130d6cbd7d16b944b004');
        // var address = Address.fromPublicKey(pubkey, 'livenet');
        // address.toString().should.equal('19gH5uhqY6DKrtkU66PsZPUZdzTd11Y7ke');
        let pubkey_hex = "0285e9737a74c30a873f74df05124f2aa6f53042c2fc0a130d6cbd7d16b944b004";
        let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
        let x = addr_bitcoin_fork(
            &pubkey_vec[..],
            AddrNetwork::BitcoinMainnet,
            AddrHashKind::P2PKH,
            false,
        );
        assert_eq!("19gH5uhqY6DKrtkU66PsZPUZdzTd11Y7ke", x);
    }

    #[test]
    fn test_bitcoin_addr1_from_key() {
        let pubkey = secp256k1_to_pubkey("0000000000000000000000000000000000000000000000000000000000000001".to_string());
        let x = addr_bitcoin_fork(
            &pubkey.serialize_compressed(),
            AddrNetwork::BitcoinMainnet,
            AddrHashKind::P2PKH,
            false,
        );
        assert_eq!("1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH", x);
    }

    #[test]
    fn test_bitcoin_addr_hash() {
        // https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/test/fixtures/address.json
        // pubkey_hash_script = 76a91465a16059864a2fdbc7c99a4723a8395bc6f188eb88ac
        // pubkey_hash_script = 76a914 + 65a16059864a2fdbc7c99a4723a8395bc6f188eb + 88ac
        // scriptPubKey: OP_DUP(0x74) OP_HASH160(0xA9) bytes-to-push(0x14) <pubKeyHash> OP_EQUALVERIFY(0x88) OP_CHECKSIG(0xAC)
        //
        // Standard Transaction to Bitcoin address (pay-to-pubkey-hash)
        // https://en.bitcoin.it/wiki/Script#Standard_Transaction_to_Bitcoin_address_.28pay-to-pubkey-hash.29
        //
        //
        // https://www.blockchain.com/btc/address/1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i
        let pubkey_hash = "65a16059864a2fdbc7c99a4723a8395bc6f188eb";

        let pubkey_vec: Vec<u8> = pubkey_hash.from_hex().unwrap();
        let x = addr_bitcoin_fork(
            &pubkey_vec[..],
            AddrNetwork::BitcoinMainnet,
            AddrHashKind::P2PKH,
            true,
        );
        assert_eq!("1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i", x);
    }

    #[test]
    fn test_bitcoin_addr_p2sh_hash() {
        // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch07.asciidoc
        // https://en.bitcoin.it/wiki/Pay_to_script_hash
        // https://github.com/bitcoin/bips/blob/master/bip-0016.mediawiki
        // https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/test/fixtures/address.json
        // "hash": "cd7b44d0b03f2d026d1e586d7ae18903b0d385f6",
        // "base58check": "3LRW7jeCvQCRdPF8S3yUCfRAx4eqXFmdcr",
        // "script": "OP_HASH160 cd7b44d0b03f2d026d1e586d7ae18903b0d385f6 OP_EQUAL"
        let script_hash = "cd7b44d0b03f2d026d1e586d7ae18903b0d385f6";

        let script_vec: Vec<u8> = script_hash.from_hex().unwrap();
        let x = addr_bitcoin_fork(
            &script_vec[..],
            AddrNetwork::BitcoinMainnet,
            AddrHashKind::P2SH,
            true,
        );
        assert_eq!("3LRW7jeCvQCRdPF8S3yUCfRAx4eqXFmdcr", x);
    }

    #[test]
    fn test_bitcoin_script_to_hex() {
        let cmdline =
            "OP_DUP OP_HASH160 788464014149d93b4a6135f3d665a0a2d743e6c3 OP_EQUALVERIFY OP_CHECKSIG";
        let sh_hex = bitcoin_script_to_hex(cmdline.to_string());
        assert_eq!("76A9788464014149D93B4A6135F3D665A0A2D743E6C388AC", sh_hex);
    }

    // TODO
    fn test_bitcoin_addr_p2sh_script() {
        // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch07.asciidoc
        // Redeem Script
        // 2 PubKey1 PubKey2 PubKey3 3 CHECKMULTISIG
        // hash160(script)
        assert_eq!("3LRW7jeCvQCRdPF8S3yUCfRAx4eqXFmdcr", "TODO");
    }

    #[test]
    fn test_litecoin_addr() {
        // https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/test/fixtures/address.json
        // https://bitcoin.stackexchange.com/questions/65282/how-is-a-litecoin-address-generated
        let pubkey_hash = "6ac624143d19a3c91d2ac5605f0aebdfeac5b826";
        let pubkey_vec: Vec<u8> = pubkey_hash.from_hex().unwrap();
        let x = addr_bitcoin_fork(
            &pubkey_vec[..],
            AddrNetwork::LitecoinMainnet,
            AddrHashKind::P2PKH,
            true,
        );
        assert_eq!("LUxXFcwXFPpRZdMv4aYu6bDwPdC2skQ5YW", x);
    }
    #[test]
    fn test_dogecoin_addr() {
        // bitcoinbook/appdx-pycoin.asciidoc at develop · bitcoinbook/bitcoinbook
        // https://github.com/bitcoinbook/bitcoinbook/blob/develop/appdx-pycoin.asciidoc#key-utility-ku
        let pubkey_hash = "751e76e8199196d454941c45d1b3a323f1433bd6";
        let pubkey_vec: Vec<u8> = pubkey_hash.from_hex().unwrap();
        let x = addr_bitcoin_fork(
            &pubkey_vec[..],
            AddrNetwork::DogecoinMainnet,
            AddrHashKind::P2PKH,
            true,
        );
        assert_eq!("DFpN6QqFfUm3gKNaxN6tNcab1FArL9cZLE", x);
    }

    fn test_to_hex() {
        assert_eq!("foobar".as_bytes().to_hex::<String>(), "666f6f626172");
    }

    #[test]
    fn test_ethereum_addr1() {
        // https://github.com/ethereumbook/ethereumbook/blob/develop/04keys-addresses.asciidoc#ethereum-addresses
        let pubkey_hex = "046e145ccef1033dea239875dd00dfb4fee6e3348b84985c92f103444683bae07b83b5c38e5e2b0c8529d7fa3f64d46daa1ece2d9ac14cab9477d042c84c32ccd0";
        let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
        let addr = addr_ethereum_fork(&pubkey_vec[..], false);
        let eip55_addr = addr_ethereum_fork(&pubkey_vec[..], true);
        assert_eq!("001d3f1ef827552ae1114027bd3ecf1f086ba0f9", addr);
        assert_eq!("001d3F1ef827552Ae1114027Bd3ECF1f086bA0F9", eip55_addr);
    }

    #[test]
    fn test_ethereum_addr1_key() {
        let pubkey : Secp256k1PublicKey = secp256k1_to_pubkey("0000000000000000000000000000000000000000000000000000000000000001".to_string());
        let x = addr_ethereum_fork(&pubkey.serialize()[..], true);
        let y = addr_ethereum_fork(&pubkey.serialize()[..], false);
        assert_eq!("7e5F4552091a69125d5DfCb7b8C2659029395Bdf", x);
        assert_eq!("7e5f4552091a69125d5dfcb7b8c2659029395bdf", y);
    }

    // https://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed
    // For sender 0x6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0, the contract addresses that it will create are the following:
    // nonce0= "0xcd234a471b72ba2f1ccf0a70fcaba648a5eecd8d"
    // nonce1= "0x343c43a37d37dff08ae8c4a11544c718abb4fcf8"
    // nonce2= "0xf778b86fa74e846c4f0a1fbd1335fe81c00a0c91"
    // nonce3= "0xfffd933a0bc612844eaf0c6fe3e5b8e9b6c1d19c"
    #[test]
    fn test_addr_eth_contract() {
        let caller = "6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0";
        let caller_vec: Vec<u8> = caller.from_hex().unwrap();
        let addr1 = addr_eth_contract(caller_vec.clone(), vec![0x01]);
        let addr2 = addr_eth_contract(caller_vec.clone(), vec![0x02]);
        assert_eq!("343c43a37d37dff08ae8c4a11544c718abb4fcf8", addr1);
        assert_eq!("f778b86fa74e846c4f0a1fbd1335fe81c00a0c91", addr2);

        // Useless Ethereum Token UET
        // https://etherscan.io/address/0x27f706edde3aD952EF647Dd67E24e38CD0803DD6
        // Contract Address : 0x27f706edde3aD952EF647Dd67E24e38CD0803DD6
        // Contract Create Tx https://etherscan.io/tx/0x7051956b39fe654480b566cec540f735c30886635cb3d1af17e2eee25ab8fac8
        // Creator Address : 0x00d0fd20924037c2b182d0aa0b139434a0b1a081
        // nonce : 2
        let uet_creator = "00d0fd20924037c2b182d0aa0b139434a0b1a081";
        let uet_creator_vec: Vec<u8> = uet_creator.from_hex().unwrap();
        let uet_addr = addr_eth_contract(uet_creator_vec.clone(), vec![0x02]);
        assert_eq!(
            "27f706edde3aD952EF647Dd67E24e38CD0803DD6".to_ascii_lowercase(),
            uet_addr
        );
    }

    #[test]
    fn test_ripple_secp256k1_addr() {
        // https://github.com/ripple/ripple-dev-portal/blob/master/content/_code-samples/address_encoding/encode_address.js#L4
        // const pubkey_hex = '0303E20EC6B4A39A629815AE02C0A1393B9225E3B890CAE45B59F42FA29BE9668D';

        let pubkey_hex = "0303E20EC6B4A39A629815AE02C0A1393B9225E3B890CAE45B59F42FA29BE9668D";
        let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
        let x = addr_ripple(&pubkey_vec[..], false);
        assert_eq!("rnBFvgZphmN39GWzUJeUitaP22Fr9be75H", x);
    }

    #[test]
    fn test_eth_checksum() {
        let addr = "0x5699b1a504f139100b889c7280074c028eb318bb";
        let res = "5699b1a504f139100B889C7280074C028eb318bB";
        assert_eq!(eth_checksum(addr), res);
    }

    #[test]
    fn test_ed25519_keys() {
        // https://docs.rs/crate/ed25519-dalek/1.0.0-pre.1/source/tests/ed25519.rs
        // From https://tools.ietf.org/html/rfc8032#section-7.3
        // let secret_key: &[u8] = b"833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42";
        // let public_key: &[u8] = b"ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
        let pubkey : Ed25519PublicKey = ed25519_to_pubkey("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42".to_string());
        assert_eq!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf", pubkey.to_bytes().to_hex::<String>());
    }

    fn test_substrate_addr() {
    // 
    // External Address Format (SS58) · paritytech/substrate Wiki
    // https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)
    //    let pair: Pair = Pair::from_seed(&hex!(
    //       "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    //    ));
    //    let public = pair.public();
        // TODO
        // let public_target = Pair::Public::from_raw(&hex!(
        //   "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
        // ));
    //    assert_eq!(public, public_target);
    }

    fn test_from_hex_okay() {
        assert_eq!("666f6f626172".from_hex::<Vec<_>>().unwrap(), b"foobar");
    }

    #[test]
    fn test_uuid_card() {
        let card = uuid_card("a cup of coffee");
        // assert_eq!(card.id.to_string(), "c145c9f2-d1a6-4118-919f-6b97cf0927d5");
        assert_eq!(card.name, "a cup of coffee");
    }

    #[test]
    fn test_primitive_types_u256() {
        // https://github.com/paritytech/parity-common/blob/master/uint/tests/uint_tests.rs
        let e = U256([16, 0, 0, 0]);
        let ua = U256::from(16);
        assert_eq!(e, ua);
    }

}
