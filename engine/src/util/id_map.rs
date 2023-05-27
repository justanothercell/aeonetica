use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;
use crate::Id;

pub type IdMap<T> = HashMap<Id, T, IdHashBuilder>;
pub type IdSet = HashSet<Id, IdHashBuilder>;

#[derive(Default)]
pub struct IdHashBuilder;

impl BuildHasher for IdHashBuilder {
    type Hasher = IdHash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        IdHash {
            state: 0
        }
    }
}

pub struct IdHash {
    state: u64,
}

impl std::hash::Hasher for IdHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = (self.state << 8) | (byte as u64);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.state
    }
}