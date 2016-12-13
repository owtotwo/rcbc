use std::fmt;
use super::location::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    value: Option<String>, // semantic value
    location: Location,
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
    // Compounded Punctuation
    DoubleEquals,             // "=="
    NotEqualTo,               // "!="
    LessThanOrEqualTo,        // "<="
    GreaterThanOrEqualTo,     // ">="
    AddAssign,                // "+="
    SubtractAssign,           // "-="
    MultiplyAssign,           // "*="
    DivideAssign,             // "/="
    ModuloAssign,             // "%="
    AndAssign,                // "&="
    ExclusiveOrAssign,        // "^="
    OrAssign,                 // "|="
    LogicalAnd,               // "&&"
    LogicalOr,                // "||"
    LeftShift,                // "<<"
    RightShift,               // ">>"
    Increment,                // "++"
    Decrement,                // "--"
    Arrow,                    // "->"
    LeftShiftAssign,          // "<<="
    RightShiftAssign,         // ">>="
    Ellipsis,                 // "..."
    // End of file
    EOF,
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>,
            location: Location) -> Token {
        match kind {
            TokenKind::Integer 
          | TokenKind::String 
          | TokenKind::Identifier
          | TokenKind::Character
          | TokenKind::Space
          | TokenKind::BlockComment
          | TokenKind::LineComment =>
                Token {
                    kind: kind, 
                    value: value,
                    location: location,
                },
            _ =>
                Token {
                    kind: kind, 
                    value: None,
                    location: location,
                },
        }
    }

    pub fn is_special(&self) -> bool {
        match self.kind {
            TokenKind::Space | 
            TokenKind::BlockComment | 
            TokenKind::LineComment =>
                true,
            _ =>
                false,
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn image(&self) -> String {
        self.value.clone().unwrap()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TokenKind::Identifier => 
                write!(f, "<IDENTIFIER>     {:?}", self.value.as_ref().unwrap()),
            // Integer Literals
            TokenKind::Integer =>
                write!(f, "<INTEGER>        {:?}", self.value.as_ref().unwrap()),
            // Character Literals
            TokenKind::Character =>
                write!(f, "<CHAR>           {:?}", self.value.as_ref().unwrap()),
            // String Literals
            TokenKind::String =>
                write!(f, "<STRING>         {:?}", self.value.as_ref().unwrap()),
            // Whitespace (blank space, new line, horizontal tab, carriage
            //             return and form feed)
            TokenKind::Space =>
                write!(f, "<SPACES>         {:?}", self.value.as_ref().unwrap()),
            //Comment
            TokenKind::BlockComment =>
                write!(f, "<BLOCK COMMENT>  {:?}", self.value.as_ref().unwrap()),
            TokenKind::LineComment =>
                write!(f, "<Line COMMENT>   {:?}", self.value.as_ref().unwrap()),
            // Reserverd Words
            TokenKind::Void =>        "`void`".fmt(f),
            TokenKind::Char =>        "`char`".fmt(f),
            TokenKind::Short =>       "`short`".fmt(f),
            TokenKind::Int =>         "`int`".fmt(f),
            TokenKind::Long =>        "`long`".fmt(f),
            TokenKind::Struct =>      "`struct`".fmt(f),
            TokenKind::Union =>       "`union`".fmt(f),
            TokenKind::Enum =>        "`enum`".fmt(f),
            TokenKind::Static =>      "`static`".fmt(f),
            TokenKind::Extern =>      "`extern`".fmt(f),
            TokenKind::Const =>       "`const`".fmt(f),
            TokenKind::Signed =>      "`signed`".fmt(f),
            TokenKind::Unsigned =>    "`unsigned`".fmt(f),
            TokenKind::If =>          "`if`".fmt(f),
            TokenKind::Else =>        "`else`".fmt(f),
            TokenKind::Switch =>      "`switch`".fmt(f),
            TokenKind::Case =>        "`case`".fmt(f),
            TokenKind::Default =>     "`default`".fmt(f),
            TokenKind::While =>       "`while`".fmt(f),
            TokenKind::Do =>          "`do`".fmt(f),
            TokenKind::For =>         "`for`".fmt(f),
            TokenKind::Return =>      "`return`".fmt(f),
            TokenKind::Break =>       "`break`".fmt(f),
            TokenKind::Continue =>    "`continue`".fmt(f),
            TokenKind::Goto =>        "`goto`".fmt(f),
            TokenKind::Typedef =>     "`typedef`".fmt(f),
            TokenKind::Import =>      "`import`".fmt(f),
            TokenKind::Sizeof =>      "`sizeof`".fmt(f),
            // Punctuation
            TokenKind::Comma =>               "`,`".fmt(f),
            TokenKind::Colon =>               "`:`".fmt(f),
            TokenKind::Semicolon =>           "`;`".fmt(f),
            TokenKind::Equals =>              "`=`".fmt(f),
            TokenKind::Underscore =>          "`_`".fmt(f),
            TokenKind::LessThan =>            "`<`".fmt(f),
            TokenKind::GreaterThan =>         "`>`".fmt(f),
            TokenKind::OpeningBracket =>      "`[`".fmt(f),
            TokenKind::ClosingBracket =>      "`]`".fmt(f),
            TokenKind::LeftCurlyBracket =>    "`{`".fmt(f),
            TokenKind::RightCurlyBracket =>   "`}`".fmt(f),
            TokenKind::OpenParentheses =>     "`(`".fmt(f),
            TokenKind::CloseParentheses =>    "`)`".fmt(f),
            TokenKind::SingleQuote =>         "`'`".fmt(f),
            TokenKind::DoubleQuotes =>       "`\"`".fmt(f),
            TokenKind::Dot =>                 "`.`".fmt(f),
            TokenKind::Slash =>               "`/`".fmt(f),
            TokenKind::Backslash =>          "`\\`".fmt(f),
            TokenKind::Plus =>                "`+`".fmt(f),
            TokenKind::Hyphen =>              "`-`".fmt(f),
            TokenKind::QuestionMark =>        "`?`".fmt(f),
            TokenKind::ExclamationMark =>     "`!`".fmt(f),
            TokenKind::Tilde =>               "`~`".fmt(f),
            TokenKind::Number =>              "`#`".fmt(f),
            TokenKind::VerticalBar =>         "`|`".fmt(f),
            TokenKind::Asterisk =>            "`*`".fmt(f),
            TokenKind::Procenttecken =>       "`%`".fmt(f),
            TokenKind::Caret =>               "`^`".fmt(f),
            TokenKind::Ampersand =>           "`&`".fmt(f),
            TokenKind::Dollar =>              "`$`".fmt(f),
            // Compounded Punctuation
            TokenKind::DoubleEquals =>            "`==`".fmt(f),
            TokenKind::NotEqualTo =>              "`!=`".fmt(f),
            TokenKind::LessThanOrEqualTo =>       "`<=`".fmt(f),
            TokenKind::GreaterThanOrEqualTo =>    "`>=`".fmt(f),
            TokenKind::AddAssign =>               "`+=`".fmt(f),
            TokenKind::SubtractAssign =>          "`-=`".fmt(f),
            TokenKind::MultiplyAssign =>          "`*=`".fmt(f),
            TokenKind::DivideAssign =>            "`/=`".fmt(f),
            TokenKind::ModuloAssign =>            "`%=`".fmt(f),
            TokenKind::AndAssign =>               "`&=`".fmt(f),
            TokenKind::ExclusiveOrAssign =>       "`^=`".fmt(f),
            TokenKind::OrAssign =>                "`|=`".fmt(f),
            TokenKind::LogicalAnd =>              "`&&`".fmt(f),
            TokenKind::LogicalOr =>               "`||`".fmt(f),
            TokenKind::LeftShift =>               "`<<`".fmt(f),
            TokenKind::RightShift =>              "`>>`".fmt(f),
            TokenKind::Increment =>               "`++`".fmt(f),
            TokenKind::Decrement =>               "`--`".fmt(f),
            TokenKind::Arrow =>                   "`->`".fmt(f),
            TokenKind::LeftShiftAssign =>         "`<<=`".fmt(f),
            TokenKind::RightShiftAssign =>        "`>>=`".fmt(f),
            TokenKind::Ellipsis =>                "`...`".fmt(f),
            // End of file
            TokenKind::EOF =>                     "<EOF>".fmt(f),
        }
    }
}