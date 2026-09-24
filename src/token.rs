#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Comma,
    Period,
    QMark,
    LeftParen,
    RightParen,
    Colon,
    ColonDash,
    //Multiply,
    //Add,
    Schemes,
    Facts,
    Rules,
    Queries,
    Id,
    r#String,
    //Comment,
    Undefined,
    End,
}

pub struct Token {
    pub t_type: TokenType,
    pub value: String,
    pub line: u32,
}

pub fn token_to_string(token: &Token) -> String {
    format!("({:?},\"{}\",{})", token.t_type, token.value, token.line)
}
