use std::fmt;

use crate::hash;
use ::serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use base64;
use serde;
use serde::{de, ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct Entry {
    pub is_left_sibling: bool,
    pub hash: hash::Hash,
}

impl Entry {
    pub fn new(is_left_sibling: bool, hash: hash::Hash) -> Self {
        let entry = Entry {
            is_left_sibling: is_left_sibling,
            hash: hash,
        };
        entry
    }
}

// Serialize for Entry is the custom serialization implementation.
// Since plain-text SHA3 hash might not be exclusive to the ASCII set,
// we need to first base64 encoded it before serializing. This is what
// this implementation achieves.
impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Entry", 2)?;
        state.serialize_field("is_left_sibling", &self.is_left_sibling)?;
        let encoded_hash = base64::encode(self.hash);
        state.serialize_field("hash", &encoded_hash)?;
        state.end()
    }
}

// Deserialize for Entry is the custom deserialization implementation.
// Since plain-text SHA3 hash might not be exclusive to the ASCII set,
// we need to first base64 encoded it before serializing. This
// implementation assumes that the serialized hash is base64 encoded.
// As a result, it decodes it first before creating an Entry.
impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Entry, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            IsLeftSibling,
            Base64Hash,
        }

        struct EntryVisitor;

        impl<'de> Visitor<'de> for EntryVisitor {
            type Value = Entry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Entry, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let is_left_sibling = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let base64_hash: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let decoded_result = base64_to_hash(base64_hash);

                if decoded_result.is_err() {
                    return Err(de::Error::custom("hash was not base64 encoded"));
                }

                Ok(Entry::new(
                    is_left_sibling,
                    hash::Hash::from(decoded_result.unwrap()),
                ))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Entry, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut is_left_sibling_opt = None;
                let mut base64_hash_opt: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::IsLeftSibling => {
                            if is_left_sibling_opt.is_some() {
                                return Err(de::Error::duplicate_field("is_left_sibling"));
                            }
                            is_left_sibling_opt = Some(map.next_value()?);
                        }
                        Field::Base64Hash => {
                            if base64_hash_opt.is_some() {
                                return Err(de::Error::duplicate_field("hash"));
                            }
                            base64_hash_opt = Some(map.next_value()?);
                        }
                    }
                }
                let is_left_sibling = is_left_sibling_opt
                    .ok_or_else(|| de::Error::missing_field("is_left_sibling"))?;
                let base64_hash =
                    base64_hash_opt.ok_or_else(|| de::Error::missing_field("hash"))?;

                let decoded_result = base64_to_hash(base64_hash);

                if decoded_result.is_err() {
                    return Err(de::Error::custom("hash was not base64 encoded"));
                }

                Ok(Entry::new(
                    is_left_sibling,
                    hash::Hash::from(decoded_result.unwrap()),
                ))
            }
        }

        const FIELDS: &'static [&'static str] = &["is_left_sibling", "hash"];
        deserializer.deserialize_struct("Entry", FIELDS, EntryVisitor)
    }
}

fn base64_to_hash(base64: String) -> Result<hash::Hash, Box<dyn Error>> {
    let decoded_result = base64::decode(base64)?;
    Ok(hash::Hash::from(decoded_result))
}

#[derive(Default, Serialize, Deserialize)]
pub struct Proof(Vec<Entry>);

impl Proof {
    pub fn push(&mut self, is_left_sibling: bool, hash: hash::Hash) {
        self.0.push(Entry {
            is_left_sibling: is_left_sibling,
            hash: hash,
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

    pub fn get_entry_at(&self, index: usize) -> &Entry {
        return &self.0[index];
    }

    pub fn get_num_entries(&self) -> usize {
        return self.0.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;
    use crate::Tree;

    #[test]
    fn verify_works() {
        let items: Vec<&[u8]> = vec![
            test_util::OSMO,
            test_util::ION,
            test_util::WETH,
            test_util::USDC,
            test_util::AKT,
        ];

        let mt = Tree::new(&items);

        let proof = mt.find_proof(&test_util::USDC).unwrap();

        let tree_root = &mt.get_root().unwrap();

        // successfuly verify node's proof.
        assert_eq!(true, proof.verify(&test_util::USDC, tree_root));

        // fail to verify other node in tree.
        assert_eq!(false, proof.verify(&test_util::OSMO, tree_root));

        // fail to verify invalid root.
        assert_eq!(
            false,
            proof.verify(&test_util::USDC, &hash::leaf(test_util::USDC))
        );
    }
}
