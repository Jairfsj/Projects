






use std::fmt;

enum ValueType {
    Str(String),
    Int(i32),
}

impl fmt::Debug for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Str(s) => write!(f, "Str({})", s),
            ValueType::Int(i) => write!(f, "Int({})", i),
        }
    }
}

struct VariableDeclaration {
    name: String,
    value: ValueType,
}

impl fmt::Debug for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VariableDeclaration {{ name: {}, value: {:?} }}", self.name, self.value)
    }
}

type AST = Vec<VariableDeclaration>;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Var,
    Identifier(String),
    Equals,
    StringLiteral(String),
    IntLiteral(i32),
    Semicolon,
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if name == "var" {
                    tokens.push(Token::Var);
                } else {
                    tokens.push(Token::Identifier(name));
                }
            }
            '"' => {
                chars.next(); // Consume initial quote
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    match ch {
                        '"' => {
                            chars.next(); // Consume closing quote
                            break;
                        }
                        _ => string.push(chars.next().unwrap()),
                    }
                }
                tokens.push(Token::StringLiteral(string));
            }
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::IntLiteral(number.parse().unwrap()));
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            _ => {
                chars.next(); // Ignore unknown characters
            }
        }
    }

    tokens
}

fn parser(tokens: &[Token]) -> AST {
    let mut ast = Vec::new();
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        if let Token::Var = token {
            if let Some(Token::Identifier(name)) = tokens.next() {
                if let Some(Token::Equals) = tokens.next() {
                    let value = match tokens.next() {
                        Some(Token::StringLiteral(value)) => ValueType::Str(value.clone()),
                        Some(Token::IntLiteral(value)) => ValueType::Int(*value),
                        _ => panic!("Expected a string or integer literal after `=`"),
                    };
                    ast.push(VariableDeclaration {
                        name: name.clone(),
                        value,
                    });
                    if tokens.next() != Some(&Token::Semicolon) {
                        panic!("Expected `;` at the end of the statement");
                    }
                } else {
                    panic!("Expected `=` after variable name");
                }
            } else {
                panic!("Expected identifier after `var`");
            }
        }
    }

    ast
}

fn main() {
    let code = r#"
    var name = "Jairfsj";
    var age = 42;
    "#;

    let tokens = lexer(&code);
    let ast = parser(&tokens);

    println!("{:#?}", ast);
}
















































































































