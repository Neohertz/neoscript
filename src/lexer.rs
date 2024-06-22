#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Number,

    OpenCurly,
    ClosedCurly,
    OpenBracket,
    ClosedBracket,
    OpenParen,
    ClosedParen,

    Semicolon,
    Equals,
    Operator,
    Unknown,
    Dot,

    // Keywords
    Let,
    Const,
    Function,

    EOF,
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
 * Tokenize the given text.
 */
pub fn tokenize(file: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Reverse the characters to use pop().
    let rev: String = file.chars().rev().collect();
    let split = rev.trim().split("");
    let mut chars: Vec<&str> = split.collect();

    while chars.len() > 0 {
        let current: &str = chars.pop().expect("Error splitting string.");

        if is_valid_token(current) == false {
            continue;
        }

        let mut char = current.to_string();

        // As of now, this only supports single characters. Arrow functions soon? :)
        let token = match char.as_ref() {
            ";" => TokenType::Semicolon,
            "." => TokenType::Dot,
            "=" => TokenType::Equals,
            "+" | "-" | "/" | "*" | "%" => TokenType::Operator,

            "{" => TokenType::OpenCurly,
            "}" => TokenType::ClosedCurly,
            "[" => TokenType::OpenBracket,
            "]" => TokenType::ClosedBracket,
            "(" => TokenType::OpenParen,
            ")" => TokenType::ClosedParen,

            // TODO: This is a bit messy, I may fix this later.
            _ => {
                // Handles Integers
                if current.parse::<i64>().is_ok() {
                    while chars.len() > 0 {
                        let next = chars.last().unwrap();

                        if next.parse::<i64>().is_ok() {
                            char.push_str(chars.pop().unwrap());
                        } else {
                            break;
                        }
                    }

                    TokenType::Number

                // Handles identifiers and keywords.
                } else if is_alphanumeric(&char) {
                    while chars.len() > 0 {
                        let next = chars.last().unwrap();

                        if is_alphanumeric(next) {
                            char.push_str(chars.pop().unwrap());
                        } else {
                            break;
                        }
                    }

                    match char.as_ref() {
                        "const" => TokenType::Const,
                        "let" => TokenType::Let,
                        "function" => TokenType::Function,
                        _ => TokenType::Identifier,
                    }
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

    tokens.push(Token {
        token_type: TokenType::EOF,
        value: String::from(""),
    });

    return tokens;
}
