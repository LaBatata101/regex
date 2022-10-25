use regex::regex::{Error, Regex};

#[test]
fn test_regex_match_a() {
    let re = Regex::new("a").unwrap();
    assert!(re.is_match("a"));
}

#[test]
fn test_regex_match_ab_or_cd() {
    let re = Regex::new("ab|cd").unwrap();
    assert!(re.is_match("ab"));
    assert!(re.is_match("cd"));
}

#[test]
fn test_regex_dont_match_ab() {
    let re = Regex::new("ab").unwrap();
    assert!(!re.is_match("aa"));
}

#[test]
fn test_regex_match_zero_or_more_ab_or_cd() {
    let re = Regex::new("(ab|cd)*").unwrap();
    assert!(re.is_match(""));
    assert!(re.is_match("ab"));
    assert!(re.is_match("abab"));
    assert!(re.is_match("cd"));
    assert!(re.is_match("cdcdcd"));
}

#[test]
fn test_regex_match_one_or_more_ab_or_cd() {
    let re = Regex::new("(ab|cd)+").unwrap();
    assert!(!re.is_match(""));
    assert!(re.is_match("ababababababab"));
    assert!(re.is_match("cd"));
}

#[test]
fn test_regex_match_multiple_closures() {
    let re = Regex::new("((ab|cd)*|(fg|h)j)+").unwrap();
    assert!(re.is_match(""));
    assert!(re.is_match("ab"));
    assert!(re.is_match("ababcdcd"));
    assert!(re.is_match("hj"));
    assert!(re.is_match("fgj"));
    assert!(re.is_match("fgjfgjfgj"));
    assert!(re.is_match("abababcdfgjfgjfgj"));
}

#[test]
fn test_regex_match_range() {
    let re = Regex::new("[a-z]").unwrap();
    assert!(re.is_match("a"));
    assert!(re.is_match("g"));
    assert!(re.is_match("h"));
    assert!(re.is_match("z"));
}

#[test]
fn test_regex_match_range2() {
    let re = Regex::new("[a-z]+").unwrap();
    assert!(re.is_match("a"));
    assert!(re.is_match("ggggg"));
    assert!(re.is_match("hh"));
    assert!(re.is_match("zzzzzzzzzzzzzz"));
}

#[test]
fn test_regex_match_range3() {
    let re = Regex::new("1[a-z]+").unwrap();
    assert!(re.is_match("1a"));
    assert!(re.is_match("1ggggg"));
    assert!(re.is_match("1hh"));
    assert!(re.is_match("1zzzzzzzzzzzzzz"));
}

#[test]
fn test_regex_match_range4() {
    let re = Regex::new("[a-zA-Z0-9]+").unwrap();

    assert!(re.is_match("a"));
    assert!(re.is_match("ddddd"));
    assert!(re.is_match("1hh"));
    assert!(re.is_match("1zzzzzzzzzzzzzz"));
    assert!(re.is_match("42"));
    assert!(re.is_match("AAAAAAA"));
    assert!(re.is_match("000000000000000000000"));

    assert!(!re.is_match(""));
    assert!(!re.is_match("AAAAAAA!"));
}

#[test]
fn test_regex_match_email() {
    let re = Regex::new("[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+").unwrap();

    assert!(re.is_match("example.samplemail@gmail.com"));
    assert!(!re.is_match("sample?examplemail@gmail.com"));
}

#[test]
fn test_regex_match_a_or_empty_string() {
    let re = Regex::new("a|"); // TODO: handle the case "|a"

    assert!(re.is_ok());

    let re = re.unwrap();
    assert!(re.is_match(""));
    assert!(re.is_match("a"));
}

#[test]
fn test_invalid_character_class_regex() {
    let re = Regex::new("[abc");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax("Brackets at position 0 doesn't have a closing brackets!".to_string())
    )
}

#[test]
fn test_invalid_character_class_regex2() {
    let re = Regex::new("[");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax("Invalid character class: missing closing bracket!".to_string())
    )
}

#[test]
fn test_invalid_range_regex() {
    let re = Regex::new("[z-a]");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::InvalidRange("Invalid Range: \"z\" is bigger than \"a\"!".to_string())
    )
}

#[test]
fn test_invalid_group_regex() {
    let re = Regex::new("(zxv");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax("Parenthesis at position 0 doesn't have a closing parenthesis!".to_string())
    )
}

#[test]
fn test_invalid_group_regex2() {
    let re = Regex::new("(");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax("Invalid group: missing closing parenthesis!".to_string())
    )
}

#[test]
fn test_invalid_closure_regex() {
    let re = Regex::new("*");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax(
            "Invalid Closure: ClosureStar operator needs a preceding literal, e.g. \"a*\", \"(ab)*\", \"(a|c)*\"."
                .to_string()
        )
    )
}

#[test]
fn test_invalid_union_regex() {
    let re = Regex::new("|");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax(
            "Invalid Union: the union operator \"|\" needs to be between two literals, e.g. \"ab|cd\", \"a|z\", \"1*|0*\".".to_string()
        )
    )
}

#[test]
fn test_invalid_closurestar_followed_by_closurestar_regex() {
    let re = Regex::new("a**");

    assert!(re.is_err());
    assert_eq!(
        re.unwrap_err(),
        Error::Syntax(
            "Invalid Closure: ClosureStar operator can't be followed by another Closure Star operator".to_string()
        )
    )
}
