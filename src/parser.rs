#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Semicolon,
    Unknown,
}

struct Token {
    token_type: TokenType,
    value: String,
}

/**
 * Returns true if the token is usable, false if not.
 */
fn is_valid_token(char: &str) -> bool {
    return match char {
        "\r" => false,
        "\t" => false,
        "\n" => false,
        "" => false,
        _ => true,
    };
}

/**
 * Parse the given text.
 */
pub fn parse(str: &String) {
    let mut tokens: Vec<Token> = Vec::new();

    // Reverse the characters to increase performance when popping off the vector.
    let rev: String = str.chars().rev().collect();
    let split = rev.trim().split("");
    let mut collection: Vec<&str> = split.collect();

    while collection.len() > 0 {
        let res = collection.pop().expect("Error splitting string.");

        if is_valid_token(res) == false {
            continue;
        }

        let char = res.to_string();

        match char {
            _ => tokens.push(Token {
                token_type: TokenType::Unknown,
                value: char,
            }),
        }
    }

    // Debug print
    for token in tokens.iter() {
        println!("{:?}, {:?}", token.token_type, token.value)
    }
}
