use crate::hash;

pub struct Entry{
    is_left_sibling: bool,
    hash: hash::Hash
}

pub struct Proof(Vec<Entry>);

impl Proof {
    pub fn push(&mut self, is_left_sibling: bool, hash: hash::Hash) {
        self.0.push(Entry{
            is_left_sibling: is_left_sibling,
            hash: hash
        })
    }

    pub fn verify<T: AsRef<[u8]>>(&self, data: &T, root: &hash::Hash) -> bool {
        let initial_hash: hash::Hash = hash::leaf(data.as_ref());

        let result = self.0.iter().try_fold(initial_hash, |cur_hash, entry| {
            let is_entry_left: bool = entry.is_left_sibling;
            if is_entry_left {
                Some(hash::branch(&entry.hash, &cur_hash))
            } else {
                Some(hash::branch(&cur_hash, &entry.hash))
            }
        });

        if result.is_none() {
            return false;
        }

        return result.unwrap().eq(root);
    }
}


