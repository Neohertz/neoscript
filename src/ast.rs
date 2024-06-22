use std::borrow::Borrow;

use crate::lexer::{Token, TokenType};

/**
 * Eat the next token.
 */
fn eat(vec: &mut Vec<Token>) -> Token {
    return vec.pop().unwrap();
}

/**
 * Take a peek at the next token.
 */
fn peek(vec: &Vec<Token>) -> &Token {
    return vec.last().unwrap();
}

/**
 * Expects a token of a given type. If the next token is not equal to the expected type, it will error.
 */
fn expect_and_eat(vec: &mut Vec<Token>, token_type: TokenType) -> Result<Token, TokenType> {
    let res = peek(vec);

    if token_type != res.token_type {
        return Err(token_type);
    }

    Ok(vec.pop().unwrap())
}

/**
 * Tokens are eaten.
 */
pub fn create(mut tokens: Vec<Token>) {
    // Reverse the tokens again to ensure that we can pop them.
    tokens = tokens.into_iter().rev().collect();

    while tokens.len() > 0 {
        // to be impl
        let token = eat(&mut tokens);
        println!("{:?}, {:?}", token.token_type, token.value);
    }
}
