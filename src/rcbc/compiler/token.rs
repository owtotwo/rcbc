#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    value: Option<String>, // semantic value
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    // Integer Literals
    Integer,
    // Character Literals
    Character,
    // String Literals
    String,
    // Whitespace (blank space, new line, horizontal tab, carriage
    //             return and form feed)
    Space,
    //Comment
    BlockComment,
    LineComment,
    // Reserverd Words
    Void,
    Char,
    Short,
    Int,
    Long,
    Struct,
    Union,
    Enum,
    Static,
    Extern,
    Const,
    Signed,
    Unsigned,
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    For,
    Return,
    Break,
    Continue,
    Goto,
    Typedef,
    Import,
    Sizeof,
    // Punctuation
    Comma,               // ','
    Colon,               // ':'
    Semicolon,           // ';'
    Equals,              // '='
    Underscore,          // '_'
    LessThan,            // '<'
    GreaterThan,         // '>'
    OpeningBracket,      // '['
    ClosingBracket,      // ']'
    LeftCurlyBracket,    // '{'
    RightCurlyBracket,   // '}'
    OpenParentheses,     // '('
    CloseParentheses,    // ')'
    SingleQuote,         // '''
    DoubleQuotes,        // '"'
    Dot,                 // '.'
    Slash,               // '/'
    Backslash,           // '\'
    Plus,                // '+'
    Hyphen,              // '-'
    QuestionMark,        // '?'
    ExclamationMark,     // '!'
    Tilde,               // '~'
    Number,              // '#'
    VerticalBar,         // '|'
    Asterisk,            // '*'
    Procenttecken,       // '%'
    Caret,               // '^'
    Ampersand,           // '&'
    Dollar,              // '$'
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>) -> Token {
        match kind {
            TokenKind::Integer 
          | TokenKind::String 
          | TokenKind::Identifier
          | TokenKind::Character =>
                Token { kind: kind, value: value },
            _ =>
                Token { kind: kind, value: None },
        }
    }
}