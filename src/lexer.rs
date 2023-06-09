use crate::token_enum::Token;
use std::iter::Peekable;

pub fn lex(code: &str) -> Peekable<std::vec::IntoIter<Token>> {
    let mut tokens = Vec::new();
    let mut code_iterator = code.chars().peekable();

    while let Some(&c) = code_iterator.peek() {
        if c.is_whitespace() {
            code_iterator.next();
        } else if c.is_digit(10) {
            let mut n = c.to_digit(10).unwrap() as usize;
            code_iterator.next();
            while let Some(&d) = code_iterator.peek() {
                if d.is_digit(10) {
                    n = n * 10 + d.to_digit(10).unwrap() as usize;
                    code_iterator.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Integer(n));
        } else if c.is_alphabetic() {
            let mut identifier = String::new();
            identifier.push(c);
            code_iterator.next();
            while let Some(&d) = code_iterator.peek() {
                if d.is_alphanumeric() || d == '_' {
                    identifier.push(d);
                    code_iterator.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Identifier(identifier));
        } else {
            match c {
                '(' => {
                    tokens.push(Token::OpenParenthesis);
                    code_iterator.next();
                }
                ')' => {
                    tokens.push(Token::ClosedParenthesis);
                    code_iterator.next();
                }
                '{' => {
                    tokens.push(Token::OpenBracket);
                    code_iterator.next();
                }
                '}' => {
                    tokens.push(Token::ClosedBracket);
                    code_iterator.next();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    code_iterator.next();
                }
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    tokens.into_iter().peekable()
}
