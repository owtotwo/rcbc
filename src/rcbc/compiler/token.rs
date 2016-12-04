#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    value: Option<String>, // semantic value
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    ReserverdWord,
    Comma,
    // ...
}