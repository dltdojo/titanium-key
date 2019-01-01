var bitcoin = require("bitcoinjs-lib");
// const keyPair = bitcoin.ECPair.makeRandom({})
const target = "00000000000000000009033885dd0de07ce3f4142aacd8dbc29512a6b9c4beb6 blah blah blah 00000000000000000009033885dd0de07ce3f4142aacd8dbc29512a6b9c4beb6"
const keyPair = bitcoin.ECPair.fromPrivateKey(bitcoin.crypto.sha256(target))
const { address } = bitcoin.payments.p2pkh({ pubkey: keyPair.publicKey })
console.log(address)
// 14MVMq277LkXQJwiGGc3LeixamGSiMMnWm