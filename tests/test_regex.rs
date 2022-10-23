use grep_clone::regex::Regex;

#[test]
fn test_regex_match_a() {
    let re = Regex::new("a");
    assert!(re.is_match("a"));
}

#[test]
fn test_regex_match_ab_or_cd() {
    let re = Regex::new("ab|cd");
    assert!(re.is_match("ab"));
    assert!(re.is_match("cd"));
}

#[test]
fn test_regex_dont_match_ab() {
    let re = Regex::new("ab");
    assert!(!re.is_match("aa"));
}

#[test]
fn test_regex_match_zero_or_more_ab_or_cd() {
    let re = Regex::new("(ab|cd)*");
    assert!(re.is_match(""));
    assert!(re.is_match("ab"));
    assert!(re.is_match("abab"));
    assert!(re.is_match("cd"));
    assert!(re.is_match("cdcdcd"));
}

#[test]
fn test_regex_match_one_or_more_ab_or_cd() {
    let re = Regex::new("(ab|cd)+");
    assert!(!re.is_match(""));
    assert!(re.is_match("ababababababab"));
    assert!(re.is_match("cd"));
}

#[test]
fn test_regex_match_multiple_closures() {
    let re = Regex::new("((ab|cd)*|(fg|h)j)+");
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
    let re = Regex::new("[a-z]");
    assert!(re.is_match("a"));
    assert!(re.is_match("g"));
    assert!(re.is_match("h"));
    assert!(re.is_match("z"));
}

#[test]
fn test_regex_match_range2() {
    let re = Regex::new("[a-z]+");
    assert!(re.is_match("a"));
    assert!(re.is_match("ggggg"));
    assert!(re.is_match("hh"));
    assert!(re.is_match("zzzzzzzzzzzzzz"));
}

#[test]
fn test_regex_match_range3() {
    let re = Regex::new("1[a-z]+");
    assert!(re.is_match("1a"));
    assert!(re.is_match("1ggggg"));
    assert!(re.is_match("1hh"));
    assert!(re.is_match("1zzzzzzzzzzzzzz"));
}

#[test]
fn test_regex_match_range4() {
    let re = Regex::new("[a-zA-Z0-9]+");

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
    let re = Regex::new("[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+");
    re.debug_save_automata_to_file("regex.svg");

    assert!(re.is_match("example.samplemail@gmail.com"));
    assert!(!re.is_match("sample?examplemail@gmail.com"));
}
