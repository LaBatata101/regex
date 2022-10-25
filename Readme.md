# Regex Engine implementation in Rust

Simple regex engine implementation in Rust using Automatas.

## Supported operators
- **Or** `a|b` - Matches "a" or "b".
- **And** `ab` - Matches "a" and "b", the `And` operator is implicit.
- **ClosureStar** `a*` - Matches zero or more "a".
- **ClosurePlus** `a+` - Matches one or more "a".
- **Character Class** `[a-zA-Z]` - Allow the creation of ranges and the `Or` operator is implicit inside the 
Character Class, the example is matching one literal between "a" and "z", or "A" and "Z" inclusive.
- **Ranges** `[a-z]` - Matches one literal between "a" and "z" inclusive.

## Example
```rust
use regex::regex::Regex;

fn main() {
    let re = Regex::new("[a-z]").unwrap();

    assert!(!re.is_match(""));
    assert!(!re.is_match("aa"));
    assert!(re.is_match("a"));
    assert!(re.is_match("z"));
}
```
