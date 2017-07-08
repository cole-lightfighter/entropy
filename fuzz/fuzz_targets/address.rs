#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate entropy;

use entropy::util::address::{Type, Address};
use entropy::blockdata::script::Script;
use entropy::network::constants::Network::{Bitcoin, Testnet};
use entropy::util::hash::Hash160;

fuzz_target!(|data: &[u8]| {
    // Hash160 must be created from a 20-item u8 slice
    if data.len() == 20 {
        // Create address from fuzzing data
        let _ = Address { 
            ty: Type::PubkeyHash, 
            network: Bitcoin, 
            hash: Hash160::from(data)
        };
    }
});
