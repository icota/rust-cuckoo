extern crate byteorder;
extern crate crypto;
extern crate siphasher;

use byteorder::{ByteOrder, NativeEndian};
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;
use siphasher::sip::SipHasher24;

pub struct Cuckoo {
    max_vertex: usize,
    max_edge: usize,
    cycle_length: usize,
}

impl Cuckoo {
    pub fn new(max_vertex: usize, max_edge: usize, cycle_length: usize) -> Self {
        Self {
            max_vertex,
            max_edge,
            cycle_length,
        }
    }

    pub fn verify(&self, message: &[u8], proof: &[u32]) -> bool {
        let hasher = {
            let mut blake_hasher = Blake2b::new(32);
            let mut result = Vec::new();
            blake_hasher.input(message);
            blake_hasher.result(&mut result);
            let key_0 = NativeEndian::read_u64(&result[0..8]).to_le();
            let key_1 = NativeEndian::read_u64(&result[8..16]).to_le();

            SipHasher24::new_with_keys(key_0, key_1)
        };

        unimplemented!()
    }

    pub fn solve(&self, message: Vec<u8>) -> Option<Vec<u32>> {
        unimplemented!()
    }
}
