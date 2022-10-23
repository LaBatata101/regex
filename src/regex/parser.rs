use super::lexer::{TokenTypes, Lexer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// a|b => a or b
    Union,
    /// ab => a and b
    Concatenation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// a+ => One or more a
    ClosurePlus,
    /// a* => Zero or more a
    ClosureStar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterClassBinaryOp {
    Union,
    Range
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexAST {
    Binary(Box<RegexAST>, BinaryOp, Box<RegexAST>),
    Unary(Box<RegexAST>, UnaryOp),
    /// a => matches the symbol a
    Symbol(char),
    /// [abc] or [a-zA-Z] etc
    CharacterClass(CharacterClassType)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CharacterClassType {
    Single(char),
    Binary(Box<CharacterClassType>, CharacterClassBinaryOp, Box<CharacterClassType>)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operation {
    CharacterClassBinary(CharacterClassBinaryOp),
    Binary(BinaryOp),
    Unary(UnaryOp),
    Unknow(TokenTypes),
}

impl Operation {
    fn get_character_class_binary_op(&self) -> CharacterClassBinaryOp {
        match self {
            Operation::CharacterClassBinary(op) => *op,
            op => panic!("Current Operation is not a CharacterClass binary: {:?}", op)
        }
    }

    fn get_binary_op(&self) -> BinaryOp {
        match self {
            Operation::Binary(op) => *op,
            op => panic!("Current Operation is not binary: {:?}", op)
        }
    }

    fn get_unary_op(&self) -> UnaryOp {
        match self {
            Operation::Unary(op) => *op,
            op => panic!("Current Operation is not unary: {:?}", op)
        }

    }
}

pub fn parse_regex(regex: &str) -> RegexAST {
    let mut lexer = Lexer::new(regex);
    parse_regex_expr(&mut lexer, 0)
}

// TODO: Return `Result` to handle errors in the syntax
fn parse_regex_expr(lexer: &mut Lexer, min_bp: u8) -> RegexAST {
    let mut lhs = if let Some(token) = lexer.next_token() {
        match token.ty {
            TokenTypes::Symbol(s) => RegexAST::Symbol(s),
            TokenTypes::OpenParenthesis => {
                let lhs = parse_regex_expr(lexer, 0);
                assert_eq!(
                    lexer.next_token().map(|token| token.ty),
                    Some(TokenTypes::CloseParenthesis),
                    "Parenthesis at position {} doesn't have a closing parenthesis!",
                    token.position().start
                );

                lhs
            }
            TokenTypes::OpenBracket => {
                let lhs = parse_character_class(lexer, 0);
                assert_eq!(lexer.next_token().map(|token| token.ty),
                    Some(TokenTypes::CloseBracket),
                    "Brackets at position {} doesn't have a closing match!",
                    token.position().start
                );

                RegexAST::CharacterClass(lhs)
            }
            t => panic!("Error: Unsuported token {:?}", t),
        }
    } else {
        panic!("Error: reached end of stream!");
    };

    while let Some(token) = lexer.peek_token() {
        let op = match token.ty {
            TokenTypes::Union => Operation::Binary(BinaryOp::Union),
            TokenTypes::Concatenation => Operation::Binary(BinaryOp::Concatenation),
            TokenTypes::ClosureStar => Operation::Unary(UnaryOp::ClosureStar),
            TokenTypes::ClosurePlus => Operation::Unary(UnaryOp::ClosurePlus),
            TokenTypes::OpenParenthesis | TokenTypes::CloseParenthesis => Operation::Unknow(token.ty),
            TokenTypes::Eof => return lhs,
            t => panic!("Error: Unsuported token {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next_token();

            if let Some(token) = lexer.peek_token() {
                if token.ty == TokenTypes::ClosureStar {
                    // TODO: handle this case!
                    println!("Invalid Second Closure! {:?}", token);
                }
            }

            lhs = RegexAST::Unary(Box::new(lhs), op.get_unary_op());
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next_token();
            let rhs = parse_regex_expr(lexer, r_bp);

            lhs = RegexAST::Binary(Box::new(lhs), op.get_binary_op(), Box::new(rhs));
            continue;
        }

        break;
    }

    lhs
}

fn parse_character_class(lexer: &mut Lexer, min_bp: u8) -> CharacterClassType {
    let mut lhs = if let Some(token) = lexer.next_token() {
        match token.ty {
            TokenTypes::Symbol(s) => CharacterClassType::Single(s),
            t => panic!("Invalid token {:?}", t)
        }
    } else {
        panic!("End of Stream")
    };

    while let Some(token) = lexer.peek_token() {
        let op = match token.ty {
            TokenTypes::Union => Operation::CharacterClassBinary(CharacterClassBinaryOp::Union),
            TokenTypes::Dash => Operation::CharacterClassBinary(CharacterClassBinaryOp::Range),
            TokenTypes::CloseBracket => Operation::Unknow(token.ty),
            TokenTypes::Eof => return lhs,
            t => panic!("Error: Unsuported token {:?}", t),
        };

        if let Some((lhs_bp, rhs_bp)) = infix_binding_power(op) {
            if lhs_bp < min_bp {
                break;
            }

            lexer.next_token();
            let rhs = parse_character_class(lexer, rhs_bp);
            let binary_op = op.get_character_class_binary_op();
            if binary_op == CharacterClassBinaryOp::Range {
                if let (CharacterClassType::Single(lhs), CharacterClassType::Single(rhs)) = (&lhs, &rhs) {
                    if rhs < lhs {
                        panic!("Invalid Range lhs is bigger than rhs!")
                    }
                }
            }

            lhs = CharacterClassType::Binary(Box::new(lhs), binary_op, Box::new(rhs));
            continue;
        }

        break;
    }

    lhs
}

fn infix_binding_power(op: Operation) -> Option<(u8, u8)> {
    match op {
        Operation::Binary(BinaryOp::Union) | Operation::CharacterClassBinary(CharacterClassBinaryOp::Union) => Some((1, 2)),
        Operation::Binary(BinaryOp::Concatenation) => Some((3, 3)),
        Operation::CharacterClassBinary(CharacterClassBinaryOp::Range) => Some((6, 5)),
        _ => None,
    }
}

fn postfix_binding_power(op: Operation) -> Option<(u8, ())> {
    match op {
        Operation::Unary(_) => Some((5, ())),
        _ => None,
    }
}
