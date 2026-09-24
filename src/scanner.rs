use crate::token::{TokenType, Token};

pub fn scanner(file_contents: r#String) -> Vec<Token> {
    let mut contents = file_contents.chars();
    let mut lines = 1;
    let mut tokens = Vec::new();

    while let Some(c) = contents.next() {
        if c == '\n' {
            lines += 1;
        } else if c.is_whitespace() {
            continue;
        } else if c.is_alphabetic() {
            let remaining = contents.as_str();
            let len = remaining.chars().take_while(|ch| ch.is_alphanumeric()).count();
            
            let rest_of_word = &remaining[..len];
            for _ in 0..len { contents.next(); }

            let full_word = format!("{}{}", c, rest_of_word);

            let t_type = match full_word.as_str() {
                "Queries" => TokenType::Queries,
                "Rules"   => TokenType::Rules,
                "Schemes" => TokenType::Schemes,
                "Facts"   => TokenType::Facts,
                _         => TokenType::Id,
            };

            tokens.push(Token { t_type, value: full_word, line: lines });
        } else if c == '\'' {
            let mut lookahead = contents.clone();
            let mut temp = vec![c];

            'string: loop {
                match lookahead.next() {
                    Some('\'') => {
                        if let Some('\'') = lookahead.clone().next() {
                            lookahead.next();
                            temp.push('\'');
                            temp.push('\'');
                        } else {
                            temp.push('\'');
                            break 'string;
                        }
                    }
                    Some('\n') => {
                        lines += 1;
                        temp.push('\n');
                    }
                    Some(other_char) => {
                        temp.push(other_char);
                    }
                    None => {
                        eprintln!("Unterminated string literal at line {}", lines);
                        std::process::exit(1);
                    }
                }
            }
            contents = lookahead;
            tokens.push(Token { t_type: TokenType::r#String, value: temp.into_iter().collect(), line: lines });
        } else if c == '#' {
            let mut lookahead = contents.clone();
            let mut temp = vec![c];

            if let Some('|') = lookahead.clone().next() {
                temp.push(lookahead.next().unwrap());
                
                loop {
                    match lookahead.next() {
                        Some('\n') => { lines += 1; temp.push('\n'); }
                        Some('|') => {
                            temp.push('|');
                            if let Some('#') = lookahead.clone().next() {
                                temp.push(lookahead.next().unwrap());
                                break;
                            }
                        }
                        Some(other) => temp.push(other),
                        None => break,
                    }
                }
            } else {
                while let Some(ch) = lookahead.clone().next() {
                    if ch == '\n' { break; }
                    temp.push(lookahead.next().unwrap());
                }
            }
            contents = lookahead;
            //tokens.push(Token { t_type: TokenType::COMMENT, value: temp.into_iter().collect(), line: lines });
        } else if c == ',' {
            tokens.push(Token { t_type: TokenType::Comma, value: c.to_string(), line: lines });
        } else if c == '.' {
            tokens.push(Token { t_type: TokenType::Period, value: c.to_string(), line: lines });
        } else if c == '?' {
            tokens.push(Token { t_type: TokenType::QMark, value: c.to_string(), line: lines });
        } else if c == '(' {
            tokens.push(Token { t_type: TokenType::LeftParen, value: c.to_string(), line: lines });
        } else if c == ')' {
            tokens.push(Token { t_type: TokenType::RightParen, value: c.to_string(), line: lines });
        } else if c == ':' {
            if let Some('-') = contents.clone().next() {
                contents.next();
                tokens.push(Token { t_type: TokenType::ColonDash, value: ":-".to_string(), line: lines });
            } else {
                tokens.push(Token { t_type: TokenType::Colon, value: c.to_string(), line: lines });
            }
        //} else if c == '*' {
        //    tokens.push(Token { t_type: TokenType::Multiply, value: c.to_string(), line: lines });
        //} else if c == '+' {
        //    tokens.push(Token { t_type: TokenType::Add, value: c.to_string(), line: lines });
        } else {
            tokens.push(Token { t_type: TokenType::Undefined, value: c.to_string(), line: lines });
        }
    }

    tokens.push(Token { t_type: TokenType::End, value: "EOF".to_string(), line: lines });
    tokens
} 
