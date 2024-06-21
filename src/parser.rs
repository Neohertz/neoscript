#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Number,

    OpenCurly,
    ClosedCurly,

    Semicolon,
    Equals,
    Operator,
    Unknown,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

/**
 * Returns true if the token is usable, false if not.
 */
fn is_valid_token(char: &str) -> bool {
    return match char {
        "\r" | "\t" | "\n" | " " | "" => false,
        _ => true,
    };
}

/**
 * TODO: May need to be more robust down the line.
 */
fn is_alphanumeric(char: &str) -> bool {
    return char.to_uppercase() != char.to_lowercase();
}

/**
 * Parse the given text.
 */
pub fn parse(str: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Reverse the characters to use pop().
    let rev: String = str.chars().rev().collect();
    let split = rev.trim().split("");
    let mut collection: Vec<&str> = split.collect();

    while collection.len() > 0 {
        let current: &str = collection.pop().expect("Error splitting string.");

        if is_valid_token(current) == false {
            continue;
        }

        let mut char = current.to_string();

        let token = match char.as_ref() {
            ";" => TokenType::Semicolon,
            "=" => TokenType::Equals,
            "+" | "-" | "/" | "*" | "%" => TokenType::Operator,
            "{" => TokenType::OpenCurly,
            "}" => TokenType::ClosedCurly,

            // TODO: This is a bit messy, I may fix this later.
            _ => {
                if current.parse::<i64>().is_ok() {
                    while collection.len() > 0 {
                        let next = collection.last().expect("Something is very wrong lol");

                        if next.parse::<i64>().is_ok() {
                            char.push_str(collection.pop().expect("This is impossible."));
                        } else {
                            break;
                        }
                    }

                    TokenType::Number
                } else if is_alphanumeric(&char) {
                    while collection.len() > 0 {
                        let next = collection.last().expect("Something is very wrong lol");

                        if is_alphanumeric(next) {
                            char.push_str(collection.pop().expect("This is impossible."));
                        } else {
                            break;
                        }
                    }

                    TokenType::Identifier
                } else {
                    TokenType::Unknown
                }
            }
        };

        tokens.push(Token {
            token_type: token,
            value: char,
        })
    }

    return tokens;
}
