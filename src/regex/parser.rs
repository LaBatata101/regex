use super::lexer::{Lexer, TokenTypes};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Syntax(String),
    InvalidRange(String),
}

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
    Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexAST {
    Binary(Box<RegexAST>, BinaryOp, Box<RegexAST>),
    Unary(Box<RegexAST>, UnaryOp),
    /// a => matches the symbol a
    Symbol(char),
    /// [abc] or [a-zA-Z] etc
    CharacterClass(CharacterClassType),
    EmptyString,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CharacterClassType {
    Single(char),
    Binary(Box<CharacterClassType>, CharacterClassBinaryOp, Box<CharacterClassType>),
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
            op => panic!("Current Operation is not a CharacterClass binary: {:?}", op),
        }
    }

    fn get_binary_op(&self) -> BinaryOp {
        match self {
            Operation::Binary(op) => *op,
            op => panic!("Current Operation is not binary: {:?}", op),
        }
    }

    fn get_unary_op(&self) -> UnaryOp {
        match self {
            Operation::Unary(op) => *op,
            op => panic!("Current Operation is not unary: {:?}", op),
        }
    }
}

pub fn parse_regex(regex: &str) -> Result<RegexAST, Error> {
    let mut lexer = Lexer::new(regex);
    let ast = parse_regex_expr(&mut lexer, 0)?;

    Ok(ast)
}

fn parse_regex_expr(lexer: &mut Lexer, min_bp: u8) -> Result<RegexAST, Error> {
    let token = lexer.next_token();
    // Handle literals
    let mut lhs = match token.ty {
        TokenTypes::Symbol(s) => RegexAST::Symbol(s),
        TokenTypes::OpenParenthesis => {
            if let Some(TokenTypes::Eof) = lexer.peek_token().map(|token| token.ty) {
                return Err(Error::Syntax("Invalid group: missing closing parenthesis!".to_string()));
            }

            let lhs = parse_regex_expr(lexer, 0)?;
            if lexer.next_token().ty != TokenTypes::CloseParenthesis {
                return Err(Error::Syntax(format!(
                    "Parenthesis at position {} doesn't have a closing parenthesis!",
                    token.position().start
                )));
            }

            lhs
        }
        TokenTypes::OpenBracket => {
            let lhs = parse_character_class(lexer, 0)?;
            if lexer.next_token().ty != TokenTypes::CloseBracket {
                return Err(Error::Syntax(format!(
                    "Brackets at position {} doesn't have a closing brackets!",
                    token.position().start
                )));
            }

            RegexAST::CharacterClass(lhs)
        }
        TokenTypes::ClosureStar => return Err(Error::Syntax(
            "Invalid Closure: ClosureStar operator needs a preceding literal, e.g. \"a*\", \"(ab)*\", \"(a|c)*\"."
                .to_string(),
        )),
        TokenTypes::ClosurePlus => return Err(Error::Syntax(
            "Invalid Closure: ClosurePlus operator needs a preceding literal, e.g. \"a+\", \"(ab)+\", \"(a|c)+\"."
                .to_string(),
        )),
        TokenTypes::Union => return Err(
            Error::Syntax(
                "Invalid Union: the union operator \"|\" needs to be between two literals, e.g. \"ab|cd\", \"a|z\", \"1*|0*\"."
                    .to_string()
            )
        ),
        TokenTypes::CloseParenthesis => return Err(Error::Syntax("Unmatched parenthesis.".to_string())),
        TokenTypes::CloseBracket => return Err(Error::Syntax("Unmatched bracket.".to_string())),
        TokenTypes::Eof => return Ok(RegexAST::EmptyString),
        t => panic!("Error: Unsuported token {:?}", t),
    };

    while let Some(token) = lexer.peek_token() {
        let op = match token.ty {
            TokenTypes::Union => Operation::Binary(BinaryOp::Union),
            TokenTypes::Concatenation => Operation::Binary(BinaryOp::Concatenation),
            TokenTypes::ClosureStar => Operation::Unary(UnaryOp::ClosureStar),
            TokenTypes::ClosurePlus => Operation::Unary(UnaryOp::ClosurePlus),
            TokenTypes::OpenParenthesis | TokenTypes::CloseParenthesis => Operation::Unknow(token.ty),
            TokenTypes::Eof => return Ok(lhs),
            t => panic!("Error: Unsuported token {:?}", t),
        };

        // Handle unary operations precedence
        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next_token();

            if let Some(TokenTypes::ClosureStar) = lexer.peek_token().map(|token| token.ty) {
                return Err(Error::Syntax(
                    "Invalid Closure: ClosureStar operator can't be followed by another Closure Star operator"
                        .to_string(),
                ));
            }

            lhs = RegexAST::Unary(Box::new(lhs), op.get_unary_op());
            continue;
        }

        // Handle binary operations precedence
        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next_token();

            let rhs = if op.get_binary_op() == BinaryOp::Union {
                // Handles the case where we have somethin like this "a|", this means we are
                // matching "a" or the empty string.
                if let Some(TokenTypes::Eof) = lexer.peek_token().map(|token| token.ty) {
                    RegexAST::EmptyString
                } else {
                    parse_regex_expr(lexer, r_bp)?
                }
            } else {
                parse_regex_expr(lexer, r_bp)?
            };

            lhs = RegexAST::Binary(Box::new(lhs), op.get_binary_op(), Box::new(rhs));
            continue;
        }

        break;
    }

    Ok(lhs)
}

fn parse_character_class(lexer: &mut Lexer, min_bp: u8) -> Result<CharacterClassType, Error> {
    let token = lexer.next_token();
    let mut lhs = match token.ty {
        TokenTypes::Symbol(s) => CharacterClassType::Single(s),
        TokenTypes::Eof => {
            return Err(Error::Syntax(
                "Invalid character class: missing closing bracket!".to_string(),
            ))
        }
        t => panic!("Invalid token {:?}", t),
    };

    while let Some(token) = lexer.peek_token() {
        let op = match token.ty {
            TokenTypes::Union => Operation::CharacterClassBinary(CharacterClassBinaryOp::Union),
            TokenTypes::Dash => Operation::CharacterClassBinary(CharacterClassBinaryOp::Range),
            TokenTypes::CloseBracket => Operation::Unknow(token.ty),
            TokenTypes::Eof => return Ok(lhs),
            t => panic!("Error: Unsuported token {:?}", t),
        };

        if let Some((lhs_bp, rhs_bp)) = infix_binding_power(op) {
            if lhs_bp < min_bp {
                break;
            }

            lexer.next_token();
            let rhs = parse_character_class(lexer, rhs_bp)?;
            let binary_op = op.get_character_class_binary_op();
            if binary_op == CharacterClassBinaryOp::Range {
                if let (CharacterClassType::Single(lhs), CharacterClassType::Single(rhs)) = (&lhs, &rhs) {
                    if rhs < lhs {
                        return Err(Error::InvalidRange(format!(
                            "Invalid Range: \"{lhs}\" is bigger than \"{rhs}\"!"
                        )));
                    }
                }
            }

            lhs = CharacterClassType::Binary(Box::new(lhs), binary_op, Box::new(rhs));
            continue;
        }

        break;
    }

    Ok(lhs)
}

fn infix_binding_power(op: Operation) -> Option<(u8, u8)> {
    match op {
        Operation::Binary(BinaryOp::Union) | Operation::CharacterClassBinary(CharacterClassBinaryOp::Union) => {
            Some((1, 2))
        }
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
