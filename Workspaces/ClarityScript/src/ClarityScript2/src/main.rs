#[derive(Debug)]
enum ValueType {
    Str(String),
    Int(i32),
}

#[derive(Debug)]
struct VariableDeclaration{
    name: String,
    value: ValueType,
}

type AST = Vec<VariableDeclaration>;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Var,
    Identifier(String),
    Equals,
    StringLiteral(String),
    Intliteral(i32),
    Semicolon,
}

fn lexex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); },
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                while let Some(&ch) = chars.peek() {
                    match ch {
                        'a'..='z' | 'A'..='Z' => {
                            name.push(chars.next().unwrap()); },
                        _ => break,
                    }
                }
                if name == "var"{
                    tokens.push(Token::Var);
                } else {
                    tokens.push(Token::Identifier(name));
                }
            },
            '"' => {
                chars.next();
                let mut string = String::new();
                while let Some(&ch) = chars.peek(){
                    match ch {
                        '"' => {chars.next(); break; }
                        _ch => string.push(chars.next().unwrap()),
                    }
                }
                tokens.push(Token::StringLiteral(string));
            },
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    match ch {
                        '0'..='9' => {
                            number.push(chars.next().unwrap());
                        },
                        _ => break,
                    }
                }
                
                tokens.push(Token::Intliteral(number.parse().unwrap()));

            },
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            },
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            },
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
