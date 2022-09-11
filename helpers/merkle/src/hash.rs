use std::fmt;

use sha3::{Digest, Sha3_256};

// The distinction in prefixes is needed
// to guard against second preimage attack
// with Merkle trees:
// https://flawed.net.nz/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack/
const LEAF_NODE_PREFIX: &[u8] = &[0];
const BRANCH_NODE_PREFIX: &[u8] = &[1];

pub const HASH_BYTES: usize = 32;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

#[derive(Clone, Default)]
pub struct Hasher {
    hasher: Sha3_256,
}

impl Hasher {
    fn update(&mut self, val: &[u8]) {
        self.hasher.update(val);
    }

    fn result(self) -> Hash {
        // At the time of this writing, the sha2 library is stuck on an old version
        // of generic_array (0.9.0). Decouple ourselves with a clone to our version.
        Hash(<[u8; HASH_BYTES]>::try_from(self.hasher.finalize().as_slice()).unwrap())
    }
}

pub fn leaf(data: &[u8]) -> Hash {
    hash(&[LEAF_NODE_PREFIX, data])
}

pub fn branch(left_child: &Hash, right_child: &Hash) -> Hash {
    hash(&[
        BRANCH_NODE_PREFIX,
        left_child.as_ref(),
        right_child.as_ref(),
    ])
}

fn hash(values: &[&[u8]]) -> Hash {
    let mut hasher = Hasher::default();
    for value in values {
        hasher.update(value);
    }
    hasher.result()
}
