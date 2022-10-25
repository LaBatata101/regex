use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenTypes {
    Symbol(char),
    Union,
    ClosurePlus,
    ClosureStar,
    Concatenation,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    Dash,
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
    pub ty: TokenTypes,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(ty: TokenTypes, start: usize, end: usize) -> Self {
        Self { ty, start, end }
    }

    pub fn position(&self) -> Range<usize> {
        self.start..self.end
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    index: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize_regex_str(input),
            index: 0,
        }
    }

    /// Get a Token and advance the Token stream pointer by 1
    pub fn next_token(&mut self) -> Token {
        // If we reach the end of tokens vector always return the EOF Token
        if let Some(&token) = self.tokens.get(self.index) {
            self.index += 1;
            token
        } else {
            self.index -= 1;
            self.tokens.get(self.index).copied().unwrap()
        }
    }

    /// Get a Token without advancing the Token stream
    pub fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.index).copied()
    }
}

// TODO: move this into the Lexer
pub fn tokenize_regex_str(regex: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let chars: Vec<char> = regex.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let symbol = chars[index];

        let mut current_token_type = get_token_type(symbol);
        tokens.push(Token::new(current_token_type, index, index + 1));

        if current_token_type == TokenTypes::OpenBracket {
            handle_character_class(&mut tokens, &chars, &mut index, &mut current_token_type);
        }

        if let Some(&next_symbol) = chars.get(index + 1) {
            let next_token_type = get_token_type(next_symbol);

            if matches!(
                current_token_type,
                TokenTypes::Symbol(_)
                    | TokenTypes::CloseParenthesis
                    | TokenTypes::CloseBracket
                    | TokenTypes::ClosurePlus
                    | TokenTypes::ClosureStar
            ) && matches!(
                next_token_type,
                TokenTypes::Symbol(_) | TokenTypes::OpenParenthesis | TokenTypes::OpenBracket
            ) {
                // Since the Concatenation is implicit we don't care about it's position
                tokens.push(Token::new(TokenTypes::Concatenation, 0, 0));
            }
        }

        index += 1;
    }

    tokens.push(Token::new(TokenTypes::Eof, index + 1, index + 1));

    tokens
}

/// [abc] or [a-zA-Z]
fn handle_character_class(
    tokens: &mut Vec<Token>,
    chars: &[char],
    index: &mut usize,
    current_token_type: &mut TokenTypes,
) {
    while !matches!(current_token_type, TokenTypes::CloseBracket | TokenTypes::Eof) {
        *index += 1;
        if let Some(&symbol) = chars.get(*index) {
            *current_token_type = get_token_type_for_character_classs(symbol);
            tokens.push(Token::new(*current_token_type, *index, *index + 1));

            if let Some(&next_symbol) = chars.get(*index + 1) {
                let next_token_type = get_token_type_for_character_classs(next_symbol);

                if let Some(&next_next_symbol) = chars.get(*index + 2) {
                    let next_next_token_type = get_token_type_for_character_classs(next_next_symbol);

                    // Only make dash a token-type if is between two symbols
                    if matches!(current_token_type, TokenTypes::Symbol(_))
                        && matches!(next_token_type, TokenTypes::Symbol('-'))
                        && matches!(next_next_token_type, TokenTypes::Symbol(_))
                    {
                        tokens.push(Token::new(TokenTypes::Dash, *index + 1, *index + 2));
                        *index += 1;
                        continue;
                    }
                }

                if matches!(current_token_type, TokenTypes::Symbol(_))
                    && matches!(next_token_type, TokenTypes::Symbol(_))
                {
                    // Inside brackets the Union is implicit
                    tokens.push(Token::new(TokenTypes::Union, 0, 0));
                }
            }
        } else {
            *current_token_type = TokenTypes::Eof;
            tokens.push(Token::new(*current_token_type, *index + 1, *index + 1));
        };
    }
}

// TODO: maybe move this to be a TokenTypes function
fn get_token_type(symbol: char) -> TokenTypes {
    match symbol {
        '*' => TokenTypes::ClosureStar,
        '+' => TokenTypes::ClosurePlus,
        '|' => TokenTypes::Union,
        '(' => TokenTypes::OpenParenthesis,
        ')' => TokenTypes::CloseParenthesis,
        '[' => TokenTypes::OpenBracket,
        ']' => TokenTypes::CloseBracket,
        _ => TokenTypes::Symbol(symbol),
    }
}

// TODO: maybe move this to be a TokenTypes function
// Inside of a "[]" all characters, except ']', are `TokenTypes::Symbol`
fn get_token_type_for_character_classs(symbol: char) -> TokenTypes {
    match symbol {
        ']' => TokenTypes::CloseBracket,
        _ => TokenTypes::Symbol(symbol),
    }
}
