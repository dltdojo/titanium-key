// https://github.com/paritytech/parity-bitcoin/blob/master/keys/src/address.rs

use base58::{FromBase58, ToBase58};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AddressType {
    /// Pay to PubKey Hash
    /// Common P2PKH which begin with the number 1, eg: 1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2.
    /// https://bitcoin.org/en/glossary/p2pkh-address
    P2PKH,
    /// Pay to Script Hash
    /// Newer P2SH type starting with the number 3, eg: 3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy.
    /// https://bitcoin.org/en/glossary/p2sh-address
    P2SH,
}

// https://github.com/paritytech/parity-bitcoin/blob/master/keys/src/lib.rs
use hash::{H160, H256};

/// 20 bytes long hash derived from public `ripemd160(sha256(public))`
pub type AddressHash = H160;

/// `AddressHash` with network identifier and format type
#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    /// The type of the address.
    pub kind: AddressType,
    /// The network of the address.
    pub network: BitcoinNetwork,
    /// Public key hash.
    pub hash: AddressHash,
}

// SLIP-0044 : Registered coin types for BIP-0044 https://github.com/satoshilabs/slips/blob/master/slip-0044.md
// wallet bip44 path


// https://github.com/DR-BoneZ/borker-rs/blob/95e9d9d9b9426fdcdbecd1050f96f69def708dc6/src/wallet/child.rs
// Address - Bitcoin Wiki https://en.bitcoin.it/wiki/Address
 pub fn pubkey_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(&self.mpub().serialize_compressed()[..]);
        let sha_bytes = hasher.result();

        let mut hasher = Ripemd160::new();
        hasher.input(&sha_bytes);
        let ripe_bytes = hasher.result();

        ripe_bytes.to_vec()
}

pub fn address_wallet_child_copy_hash160_checksum_type(&self, network: Network) -> String {

        // Base58 and Base58Check Encoding
        // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch04.asciidoc#base58-and-base58check-encoding
        let base58check_version_prefix: u8 = match network {
            Network::Dogecoin => 0x1E,
            Network::Litecoin => 0x30,
            Network::Bitcoin => 0x00, // Pay-to-Script-Hash Address 0x05 ? BIP-38 Encrypted Private Key 0x0142
            Network::BitcoinTestnet => 0x6f,
        };

        // Base58Check version prefix


        let mut addr_bytes: Vec<u8> = vec![base58check_version_prefix];

        // public key hash
        addr_bytes.extend(&self.pubkey_hash());

        // checksum = SHA256(SHA256(prefix+data))
        let mut hasher = Sha256::new();
        hasher.input(&addr_bytes);
        let res = hasher.result();
        let mut hasher = Sha256::new();
        hasher.input(&res);
        let chksum = hasher.result();

        addr_bytes.extend(&chksum[0..4]);

        ToBase58::to_base58(addr_bytes.as_slice())
    }