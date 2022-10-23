use crate::regex::parser::{parse_regex, BinaryOp, CharacterClassType, RegexAST, UnaryOp, CharacterClassBinaryOp};

#[test]
fn test_parse_symbol() {
    let expr = parse_regex("a");
    assert_eq!(expr, RegexAST::Symbol('a'))
}

#[test]
fn test_parse_union() {
    let expr = parse_regex("a|b");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Symbol('a')),
            BinaryOp::Union,
            Box::new(RegexAST::Symbol('b'))
        )
    )
}

#[test]
fn test_parse_concatenation() {
    let expr = parse_regex("ab");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Symbol('a')),
            BinaryOp::Concatenation,
            Box::new(RegexAST::Symbol('b'))
        )
    )
}

#[test]
fn test_parse_concatenation_and_union() {
    let expr = parse_regex("ab|cd");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Symbol('a')),
                BinaryOp::Concatenation,
                Box::new(RegexAST::Symbol('b'))
            )),
            BinaryOp::Union,
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Symbol('c')),
                BinaryOp::Concatenation,
                Box::new(RegexAST::Symbol('d'))
            ))
        )
    )
}

#[test]
fn test_parse_multiple_union() {
    let expr = parse_regex("ab|cd|ef");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('a')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('b'))
                )),
                BinaryOp::Union,
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('c')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('d'))
                ))
            )),
            BinaryOp::Union,
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Symbol('e')),
                BinaryOp::Concatenation,
                Box::new(RegexAST::Symbol('f'))
            ))
        )
    )
}

#[test]
fn test_parse_closurestar_with_parens() {
    let expr = parse_regex("(ab)*");
    assert_eq!(
        expr,
        RegexAST::Unary(
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Symbol('a')),
                BinaryOp::Concatenation,
                Box::new(RegexAST::Symbol('b'))
            )),
            UnaryOp::ClosureStar
        )
    )
}

#[test]
fn test_parse_closurestar_with_parens_and_union() {
    let expr = parse_regex("(ab)*|c*");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Unary(
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('a')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('b'))
                )),
                UnaryOp::ClosureStar
            )),
            BinaryOp::Union,
            Box::new(RegexAST::Unary(Box::new(RegexAST::Symbol('c')), UnaryOp::ClosureStar))
        )
    )
}

#[test]
fn test_parse_closurestar() {
    let expr = parse_regex("a*");
    assert_eq!(
        expr,
        RegexAST::Unary(Box::new(RegexAST::Symbol('a')), UnaryOp::ClosureStar)
    )
}

#[test]
fn test_parse_closureplus() {
    let expr = parse_regex("a+");
    assert_eq!(
        expr,
        RegexAST::Unary(Box::new(RegexAST::Symbol('a')), UnaryOp::ClosurePlus)
    )
}

#[test]
fn test_parse_closurestar_union_closureplus() {
    let expr = parse_regex("(ab)*|(cd)+");
    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Unary(
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('a')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('b'))
                )),
                UnaryOp::ClosureStar
            )),
            BinaryOp::Union,
            Box::new(RegexAST::Unary(
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('c')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('d'))
                )),
                UnaryOp::ClosurePlus
            ))
        )
    )
}

#[test]
fn test_parse_union_with_paren_and_concatenation() {
    let expr = parse_regex("(ab|cd|ef)g");

    assert_eq!(
        expr,
        RegexAST::Binary(
            Box::new(RegexAST::Binary(
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Binary(
                        Box::new(RegexAST::Symbol('a')),
                        BinaryOp::Concatenation,
                        Box::new(RegexAST::Symbol('b'))
                    )),
                    BinaryOp::Union,
                    Box::new(RegexAST::Binary(
                        Box::new(RegexAST::Symbol('c')),
                        BinaryOp::Concatenation,
                        Box::new(RegexAST::Symbol('d'))
                    ))
                )),
                BinaryOp::Union,
                Box::new(RegexAST::Binary(
                    Box::new(RegexAST::Symbol('e')),
                    BinaryOp::Concatenation,
                    Box::new(RegexAST::Symbol('f'))
                ))
            )),
            BinaryOp::Concatenation,
            Box::new(RegexAST::Symbol('g'))
        )
    )
}

#[test]
fn test_parse_character_class_range() {
    let expr = parse_regex("[a-z]");

    assert_eq!(
        expr,
        RegexAST::CharacterClass(CharacterClassType::Binary(
            Box::new(CharacterClassType::Single('a')),
            CharacterClassBinaryOp::Range,
            Box::new(CharacterClassType::Single('z'))
        ))
    )
}

#[test]
fn test_parse_character_class_range2() {
    let expr = parse_regex("[a-z]+");

    assert_eq!(
        expr,
        RegexAST::Unary(Box::new(RegexAST::CharacterClass(CharacterClassType::Binary(
            Box::new(CharacterClassType::Single('a')),
            CharacterClassBinaryOp::Range,
            Box::new(CharacterClassType::Single('z'))
        ))), UnaryOp::ClosurePlus)
    )
}

#[test]
fn test_parse_character_class_multiple_symbols() {
    let expr = parse_regex("[abc]");

    assert_eq!(
        expr,
        RegexAST::CharacterClass(CharacterClassType::Binary(
            Box::new(CharacterClassType::Binary(
                Box::new(CharacterClassType::Single('a')),
                CharacterClassBinaryOp::Union,
                Box::new(CharacterClassType::Single('b'))
            )),
            CharacterClassBinaryOp::Union,
            Box::new(CharacterClassType::Single('c'))
        ))
    )
}

#[test]
fn test_parse_character_class_with_two_ranges() {
    let expr = parse_regex("[a-zA-Z]");

    assert_eq!(
        expr,
        RegexAST::CharacterClass(CharacterClassType::Binary(
            Box::new(CharacterClassType::Binary(
                Box::new(CharacterClassType::Single('a')),
                CharacterClassBinaryOp::Range,
                Box::new(CharacterClassType::Single('z'))
            )),
            CharacterClassBinaryOp::Union,
            Box::new(CharacterClassType::Binary(
                Box::new(CharacterClassType::Single('A')),
                CharacterClassBinaryOp::Range,
                Box::new(CharacterClassType::Single('Z'))
            ))
        ))
    )
}

#[test]
fn test_parse_character_class_with_three_ranges() {
    let expr = parse_regex("[a-zA-Z0-9]");

    assert_eq!(
        expr,
        RegexAST::CharacterClass(CharacterClassType::Binary(
            Box::new(CharacterClassType::Binary(
                Box::new(CharacterClassType::Binary(
                    Box::new(CharacterClassType::Single('a')),
                    CharacterClassBinaryOp::Range,
                    Box::new(CharacterClassType::Single('z')),
                )),
                CharacterClassBinaryOp::Union,
                Box::new(CharacterClassType::Binary(
                    Box::new(CharacterClassType::Single('A')),
                    CharacterClassBinaryOp::Range,
                    Box::new(CharacterClassType::Single('Z')),
                )),
            )),
            CharacterClassBinaryOp::Union,
            Box::new(CharacterClassType::Binary(
                Box::new(CharacterClassType::Single('0')),
                CharacterClassBinaryOp::Range,
                Box::new(CharacterClassType::Single('9')),
            )),
        ))
    )
}
