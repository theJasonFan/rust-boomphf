// gen keys

//use crate::bitvector::BitVector;
use std::hash::{Hash, Hasher};
use std::convert::TryInto;
use std::num::Wrapping;

// See hash64 in pufferfish:include/BooPHF.hpp
pub fn hash_with_seed<T: Hash + ?Sized>(iter: u64, v: &T) -> u64 {
    let mut state = SimpleHash::new(iter);
    v.hash(&mut state);
    state.finish()
}

// SingleHashFunctor
struct SimpleHash {
    seed: u64,
    state: u64,
}

impl Default for SimpleHash {
    fn default() -> Self {
        Self {
            seed: 0xAAAAAAAA55555555,
            state: 0,
        }
    }
}

impl Hasher for SimpleHash {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        // TODO: a better hash function would consume byte-by-byte...
        let (word, _rest) = bytes.split_at(std::mem::size_of::<u64>());
        //*input = rest; //don't need the rest of the bytes so no need to shift pointer
        let key = u64::from_ne_bytes(word.try_into().unwrap());
        //let key = u64::from_ne_bytes(bytes[..std::mem::size_of::<u64>()]);
        self.state = Self::hash64(key, self.seed);
    }

}
impl SimpleHash{
    fn new(seed: u64) -> Self {
        Self {
            seed: seed,
            state: 0,
        }
    }
    
    fn hash64(key: u64, seed: u64) -> u64 {
        // allow overflow
        let mut hash = Wrapping(seed);
        let key = Wrapping(key);

        let init = (hash <<  7) ^  key * (hash >> 3) ^ (!((hash << 11) + (key ^ (hash >> 5))));
        hash = hash ^ init;
        hash = (!hash) + (hash << 21);
        hash = hash ^ (hash >> 24);
        hash = (hash + (hash << 3)) + (hash << 8);
        hash = hash ^ (hash >> 14);
        hash = (hash + (hash << 2)) + (hash << 4);
        hash = hash ^ (hash >> 28);
        hash = hash + (hash << 31);

        hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero() {
        let state = SimpleHash::default();
        let hash_0 = 0x6e1bccdb7aa2bc25;
        let hash = hash_with_seed(state.seed, &0u64);
        assert_eq!(hash_0, hash);
    }

    fn first10() {
        let state = SimpleHash::default();
        let true_hashes = 
                vec![0x6e1bccdb7aa2bc25,
                     0x54676a7b01425b7,
                     0x5c9be323e5ad1be1,
                     0x9567829f5e948f83,
                     0xcf71e329165c79b5,
                     0x9f1219f1bcd9d206,
                     0x6bd828b35dba940e,
                     0xf55b08c3340017c3,
                     0xd178ae94742fa575,
                     0x5dc299d49318dc6b];
        for (key, hash) in true_hashes.into_iter().enumerate() {
            let out = hash_with_seed(state.seed, &key);
            assert_eq!(hash, out);
        }
    }
}
