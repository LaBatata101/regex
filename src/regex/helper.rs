use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn calculate_hash<T: Hash>(object: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    object.hash(&mut hasher);
    hasher.finish()
}
