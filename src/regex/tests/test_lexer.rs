use crate::regex::lexer::{tokenize_regex_str, Token, TokenTypes};

#[test]
fn test_tokenize_regex_symbol() {
    let regex = "a";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('a'), 0, 1),
            Token::new(TokenTypes::Eof, 2, 2)
        ]
    )
}

#[test]
fn test_tokenize_regex_union() {
    let regex = "a|b";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('a'), 0, 1),
            Token::new(TokenTypes::Union, 1, 2),
            Token::new(TokenTypes::Symbol('b'), 2, 3),
            Token::new(TokenTypes::Eof, 4, 4)
        ]
    )
}

#[test]
fn test_tokenize_regex_closure_star() {
    let regex = "a*";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('a'), 0, 1),
            Token::new(TokenTypes::ClosureStar, 1, 2),
            Token::new(TokenTypes::Eof, 3, 3)
        ]
    )
}

#[test]
fn test_tokenize_regex_closure_plus() {
    let regex = "a+";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('a'), 0, 1),
            Token::new(TokenTypes::ClosurePlus, 1, 2),
            Token::new(TokenTypes::Eof, 3, 3)
        ]
    )
}

#[test]
fn test_tokenize_regex_concatenation() {
    let regex = "test";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('t'), 0, 1),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('e'), 1, 2),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('s'), 2, 3),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('t'), 3, 4),
            Token::new(TokenTypes::Eof, 5, 5)
        ]
    )
}

#[test]
fn test_tokenize_regex_concatenation_and_union() {
    let regex = "test|foo";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('t'), 0, 1),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('e'), 1, 2),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('s'), 2, 3),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('t'), 3, 4),
            Token::new(TokenTypes::Union, 4, 5),
            Token::new(TokenTypes::Symbol('f'), 5, 6),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('o'), 6, 7),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('o'), 7, 8),
            Token::new(TokenTypes::Eof, 9, 9)
        ]
    )
}

#[test]
fn test_tokenize_regex_with_parenthesis() {
    let regex = "(test)";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenParenthesis, 0, 1),
            Token::new(TokenTypes::Symbol('t'), 1, 2),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('e'), 2, 3),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('s'), 3, 4),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('t'), 4, 5),
            Token::new(TokenTypes::CloseParenthesis, 5, 6),
            Token::new(TokenTypes::Eof, 7, 7)
        ]
    )
}

#[test]
fn test_tokenize_regex_with_multiple_parenthesis() {
    let regex = "(test|(s|e)*)e";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenParenthesis, 0, 1),
            Token::new(TokenTypes::Symbol('t'), 1, 2),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('e'), 2, 3),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('s'), 3, 4),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('t'), 4, 5),
            Token::new(TokenTypes::Union, 5, 6),
            Token::new(TokenTypes::OpenParenthesis, 6, 7),
            Token::new(TokenTypes::Symbol('s'), 7, 8),
            Token::new(TokenTypes::Union, 8, 9),
            Token::new(TokenTypes::Symbol('e'), 9, 10),
            Token::new(TokenTypes::CloseParenthesis, 10, 11),
            Token::new(TokenTypes::ClosureStar, 11, 12),
            Token::new(TokenTypes::CloseParenthesis, 12, 13),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('e'), 13, 14),
            Token::new(TokenTypes::Eof, 15, 15)
        ]
    )
}

#[test]
fn test_tokenize_parenthesis_concatenation() {
    let regex = "(a|b)(c|d)";
    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenParenthesis, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Union, 2, 3),
            Token::new(TokenTypes::Symbol('b'), 3, 4),
            Token::new(TokenTypes::CloseParenthesis, 4, 5),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::OpenParenthesis, 5, 6),
            Token::new(TokenTypes::Symbol('c'), 6, 7),
            Token::new(TokenTypes::Union, 7, 8),
            Token::new(TokenTypes::Symbol('d'), 8, 9),
            Token::new(TokenTypes::CloseParenthesis, 9, 10),
            Token::new(TokenTypes::Eof, 11, 11),
        ]
    )
}

#[test]
fn test_tokenize_char_range_lowercase() {
    let regex = "[a-z]";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Dash, 2, 3),
            Token::new(TokenTypes::Symbol('z'), 3, 4),
            Token::new(TokenTypes::CloseBracket, 4, 5),
            Token::new(TokenTypes::Eof, 6, 6),
        ]
    )
}

#[test]
fn test_tokenize_char_range_lowercase_or_uppercase() {
    let regex = "[a-zA-Z]";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Dash, 2, 3),
            Token::new(TokenTypes::Symbol('z'), 3, 4),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('A'), 4, 5),
            Token::new(TokenTypes::Dash, 5, 6),
            Token::new(TokenTypes::Symbol('Z'), 6, 7),
            Token::new(TokenTypes::CloseBracket, 7, 8),
            Token::new(TokenTypes::Eof, 9, 9),
        ]
    )
}

#[test]
fn test_tokenize_char_range_concatenated_with_symbol() {
    let regex = "[a-zA-Z]1";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Dash, 2, 3),
            Token::new(TokenTypes::Symbol('z'), 3, 4),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('A'), 4, 5),
            Token::new(TokenTypes::Dash, 5, 6),
            Token::new(TokenTypes::Symbol('Z'), 6, 7),
            Token::new(TokenTypes::CloseBracket, 7, 8),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('1'), 8, 9),
            Token::new(TokenTypes::Eof, 10, 10),
        ]
    )
}

#[test]
fn test_tokenize_concatenate_symbol_with_char_range() {
    let regex = "1[a-zA-Z]";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::Symbol('1'), 0, 1),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::OpenBracket, 1, 2),
            Token::new(TokenTypes::Symbol('a'), 2, 3),
            Token::new(TokenTypes::Dash, 3, 4),
            Token::new(TokenTypes::Symbol('z'), 4, 5),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('A'), 5, 6),
            Token::new(TokenTypes::Dash, 6, 7),
            Token::new(TokenTypes::Symbol('Z'), 7, 8),
            Token::new(TokenTypes::CloseBracket, 8, 9),
            Token::new(TokenTypes::Eof, 10, 10),
        ]
    )
}

#[test]
fn test_tokenize_multiple_symbols_character_class() {
    let regex = "[abc]";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('b'), 2, 3),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('c'), 3, 4),
            Token::new(TokenTypes::CloseBracket, 4, 5),
            Token::new(TokenTypes::Eof, 6, 6),
        ]
    )
}

#[test]
fn test_tokenize_multiple_symbols_character_class2() {
    let regex = "[abc-]";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('b'), 2, 3),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('c'), 3, 4),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('-'), 4, 5),
            Token::new(TokenTypes::CloseBracket, 5, 6),
            Token::new(TokenTypes::Eof, 7, 7),
        ]
    )
}

#[test]
fn test_tokenize_multiple_symbols_character_class3() {
    let regex = "[abc-]+@+[0123]*";

    assert_eq!(
        tokenize_regex_str(regex),
        vec![
            Token::new(TokenTypes::OpenBracket, 0, 1),
            Token::new(TokenTypes::Symbol('a'), 1, 2),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('b'), 2, 3),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('c'), 3, 4),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('-'), 4, 5),
            Token::new(TokenTypes::CloseBracket, 5, 6),
            Token::new(TokenTypes::ClosurePlus, 6, 7),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::Symbol('@'), 7, 8),
            Token::new(TokenTypes::ClosurePlus, 8, 9),
            Token::new(TokenTypes::Concatenation, 0, 0),
            Token::new(TokenTypes::OpenBracket, 9, 10),
            Token::new(TokenTypes::Symbol('0'), 10, 11),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('1'), 11, 12),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('2'), 12, 13),
            Token::new(TokenTypes::Union, 0, 0),
            Token::new(TokenTypes::Symbol('3'), 13, 14),
            Token::new(TokenTypes::CloseBracket, 14, 15),
            Token::new(TokenTypes::ClosureStar, 15, 16),
            Token::new(TokenTypes::Eof, 17, 17),
        ]
    )
}
