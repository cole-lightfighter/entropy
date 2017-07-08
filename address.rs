#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate entropy;

use secp256k1::{self, Secp256k1};
use secp256k1::key::{PublicKey, SecretKey};
use network::constants::Network;

use entropy::util::address;

fuzz_target!(|data: &[u8]| {
    let secp = secp256k1::Secp256k1.new();

    // Create key from fuzzing data
    let k = secp256k1::key::PublicKey.from_slice(secp, data);

    let _ = address::Address.from_key(Network, k, false)
});
