use sha2::Sha256;
use ripemd160::{Digest, Ripemd160};
use base58::ToBase58;

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

pub fn bitcoin_addr(pubkey: &[u8]) -> String {
        // Base58 and Base58Check Encoding
        // https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch04.asciidoc#base58-and-base58check-encoding
        let base58check_version_prefix: u8 = 0x00;

        // Base58Check version prefix
        let mut addr_bytes: Vec<u8> = vec![base58check_version_prefix];

        // public key hash
        addr_bytes.extend(hash160(pubkey));

        // checksum = SHA256(SHA256(prefix+data))
        let chksum = sha256sha256(&addr_bytes[..]);
        addr_bytes.extend(&chksum[0..4]);
        ToBase58::to_base58(addr_bytes.as_slice())
    }

#[cfg(test)]
mod tests {
	use super::*;
	use rustc_hex::{FromHex,ToHex};
	
	#[test]
	fn test_hash160() {
		// https://github.com/bitpay/bitcore-lib/blob/master/test/crypto/hash.js#L80
		// var buf = new Buffer([0, 1, 2, 3, 253, 254, 255]);
		// hash.toString('hex').should.equal('7322e2bd8535e476c092934e16a6169ca9b707ec');
		let pubkey_hex = "00010203fdfeff";
		let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
		let x = hash160(&pubkey_vec[..]);
        assert_eq!("7322e2bd8535e476c092934e16a6169ca9b707ec", x.to_hex::<String>());
    }

	#[test]
	fn test_sha256sha256() {
		// https://github.com/bitpay/bitcore-lib/blob/master/test/crypto/hash.js#L80
		// var buf = new Buffer([0, 1, 2, 3, 253, 254, 255]);
		// hash.toString('hex').should.equal('7322e2bd8535e476c092934e16a6169ca9b707ec');
		let pubkey_hex = "00010203fdfeff";
		let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
		let x = sha256sha256(&pubkey_vec[..]);
        assert_eq!("be586c8b20dee549bdd66018c7a79e2b67bb88b7c7d428fa4c970976d2bec5ba", x.to_hex::<String>());
    }

	#[test]
	fn test_bitcoin_addr() {
		// https://github.com/bitpay/bitcore-lib/blob/master/test/address.js#L338
		// var pubkey = new PublicKey('0285e9737a74c30a873f74df05124f2aa6f53042c2fc0a130d6cbd7d16b944b004');
        // var address = Address.fromPublicKey(pubkey, 'livenet');
        // address.toString().should.equal('19gH5uhqY6DKrtkU66PsZPUZdzTd11Y7ke');
		let pubkey_hex = "0285e9737a74c30a873f74df05124f2aa6f53042c2fc0a130d6cbd7d16b944b004";
		let pubkey_vec: Vec<u8> = pubkey_hex.from_hex().unwrap();
		let x = bitcoin_addr(&pubkey_vec[..]);
        assert_eq!("19gH5uhqY6DKrtkU66PsZPUZdzTd11Y7ke", x);
    }

	#[test]
    pub fn test_to_hex() {
        assert_eq!("foobar".as_bytes().to_hex::<String>(), "666f6f626172");
    }

	#[test]
    pub fn test_from_hex_okay() {
        assert_eq!("666f6f626172".from_hex::<Vec<_>>().unwrap(),
                   b"foobar");
        assert_eq!("666F6F626172".from_hex::<Vec<_>>().unwrap(),
                   b"foobar");
    }

}