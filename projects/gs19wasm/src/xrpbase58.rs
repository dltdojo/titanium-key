// base on https://docs.rs/crate/base58/0.1.0/source/src/lib.rs
// XRP Ledger addresses are encoded using base58  with the Ripple dictionary: rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz. 
// https://github.com/ripple/ripple-dev-portal/blob/master/content/_code-samples/address_encoding/encode_address.js#L4
    
// const ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET: &'static [u8] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

const B58_DIGITS_MAP: &'static [i8] = &[
	-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
	-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
	-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
	-1, 0, 1, 2, 3, 4, 5, 6, 7, 8,-1,-1,-1,-1,-1,-1,
	-1, 9,10,11,12,13,14,15,16,-1,17,18,19,20,21,-1,
	22,23,24,25,26,27,28,29,30,31,32,-1,-1,-1,-1,-1,
	-1,33,34,35,36,37,38,39,40,41,42,43,-1,44,45,46,
	47,48,49,50,51,52,53,54,55,56,57,-1,-1,-1,-1,-1,
];

/// Errors that can occur when decoding base58 encoded string.
#[derive(Debug, PartialEq)]
pub enum FromBase58Error {
	/// The input contained a character which is not a part of the base58 format.
	InvalidBase58Character(char, usize),
	/// The input had invalid length.
	InvalidBase58Length,
}

/// A trait for converting a value to base58 encoded string.
pub trait ToXrpBase58 {
	/// Converts a value of `self` to a base58 value, returning the owned string.
	fn to_base58(&self) -> String;
}

/// A trait for converting base58 encoded values.
pub trait FromXrpBase58 {
	/// Convert a value of `self`, interpreted as base58 encoded data, into an owned vector of bytes, returning a vector.
	fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error>;
}

impl ToXrpBase58 for [u8] {
	fn to_base58(&self) -> String {
		let zcount = self.iter().take_while(|x| **x == 0).count();
		let size = (self.len() - zcount) * 138 / 100 + 1;
		let mut buffer = vec![0u8; size];

		let mut i = zcount;
		let mut high = size - 1;

		while i < self.len() {
			let mut carry = self[i] as u32;
			let mut j = size - 1;

			while j > high || carry != 0 {
				carry += 256 * buffer[j] as u32;
				buffer[j] = (carry % 58) as u8;
				carry /= 58;

				// in original trezor implementation it was underflowing
				if j  > 0 {
					j -= 1;
				}
			}

			i += 1;
			high = j;
		}

		let mut j = buffer.iter().take_while(|x| **x == 0).count();

		let mut result = String::new();
		for _ in 0..zcount {
			//result.push('1');
			result.push('r');
		}

		while j < size {
			result.push(ALPHABET[buffer[j] as usize] as char);
			j += 1;
		}

		result
	}
}

impl FromXrpBase58 for str {
	fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error> {
		let mut bin = [0u8; 132];
		let mut out = [0u32; (132 + 3) / 4];
		let bytesleft = (bin.len() % 4) as u8;
		let zeromask = match bytesleft {
			0 => 0u32,
			_ => 0xffffffff << (bytesleft * 8),
		};

		// TODO ? change to 'r' ??
		let zcount = self.chars().take_while(|x| *x == '1').count();
		let mut i = zcount;
		let b58: Vec<u8> = self.bytes().collect();

		while i < self.len() {
			if (b58[i] & 0x80) != 0 {
				// High-bit set on invalid digit
				return Err(FromBase58Error::InvalidBase58Character(b58[i] as char, i));
			}

			if B58_DIGITS_MAP[b58[i] as usize] == -1 {
				// // Invalid base58 digit
				return Err(FromBase58Error::InvalidBase58Character(b58[i] as char, i));
			}

			let mut c = B58_DIGITS_MAP[b58[i] as usize] as u64;
			let mut j = out.len();
			while j != 0 {
				j -= 1;
				let t = out[j] as u64 * 58 + c;
				c = (t & 0x3f00000000) >> 32;
				out[j] = (t & 0xffffffff) as u32;
			}

			if c != 0 {
				// Output number too big (carry to the next int32)
				return Err(FromBase58Error::InvalidBase58Length);
			}

			if (out[0] & zeromask) != 0 {
				// Output number too big (last int32 filled too far)
				return Err(FromBase58Error::InvalidBase58Length);
			}

			i += 1;
		}

		let mut i = 1;
		let mut j = 0;

		bin[0] = match bytesleft {
			3 => ((out[0] & 0xff0000) >> 16) as u8,
			2 => ((out[0] & 0xff00) >> 8) as u8,
			1 => {
				j = 1;
				(out[0] & 0xff) as u8
			},
			_ => {
				i = 0;
				bin[0]
			}
		};

		while j < out.len() {
			bin[i] = ((out[j] >> 0x18) & 0xff) as u8;
			bin[i + 1] = ((out[j] >> 0x10) & 0xff) as u8;
			bin[i + 2] = ((out[j] >> 8) & 0xff) as u8;
			bin[i + 3] = ((out[j] >> 0) & 0xff) as u8;
			i += 4;
			j += 1;
		}

		let leading_zeros = bin.iter().take_while(|x| **x == 0).count();
		Ok(bin[leading_zeros - zcount..].to_vec())
	}
}