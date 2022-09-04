use merkle_light::merkle::{MerkleTree};
use merkle_light_derive::Hashable;
use merkle_light::hash::{Algorithm, Hashable};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use sha2::{Sha256, Sha512, Digest, Sha256VarCore};

struct SHA256(Sha256);

impl SHA256 {
    fn new() -> SHA256 {
        SHA256(Sha256::new())
    }
}

impl Default for SHA256 {
    fn default() -> SHA256 {
        SHA256::new()
    }
}

impl Hasher for SHA256{
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.update(msg);
    }

    #[inline]
    fn finish(&self) -> u64 {
        let res: u64 = 
        self.0.finalize_into(out: &mut Output<Self>)();
        return 0
    }
}

#[derive(Hashable, Debug)]
pub struct Leaf {
   address: String,
   amount: String,
 }

//  impl Algorithm<[u8; 32]> for SHA256 {
//     #[inline]
//     fn hash(&mut self) -> [u8; 32] {
//         let mut h = [0u8; 32];
//         let result = self.0.
//         h
//     }

//     #[inline]
//     fn reset(&mut self) {
//         self.0.reset();
//     }
// }


pub fn generate_merkle_root(data: Vec<Leaf>)  {
    // let t: MerkleTree<Leaf, Sha256> = MerkleTree::from_data(data);
}
