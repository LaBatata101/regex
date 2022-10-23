use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn calculate_hash<T: Hash>(object: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    object.hash(&mut hasher);
    hasher.finish()
}

pub fn reduce_string_to_range(strings: &[String]) -> String {
    for str in strings {
        dbg!(str
            .chars()
            .skip(1)
            .zip(str.chars().skip(2))
            .collect::<Vec<(char, char)>>());
        // for (c1, c2) in str.chars().zip(str.chars().skip(1)) {}
    }
    todo!()
}

#[cfg(test)]
mod test_helpers {
    use super::reduce_string_to_range;

    #[test]
    fn test_reduce_string_to_range() {
        let str: String = ('a'..='z').collect();
        reduce_string_to_range(&[str]);
    }
}
