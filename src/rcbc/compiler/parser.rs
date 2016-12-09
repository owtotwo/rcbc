use super::token::{Token, TokenKind};
use super::ast::*;
use std::result;
use std::fmt;
use std::slice::Iter;

type Result<'a, T> = result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    iter: Iter<'a, Token<'a>>,
    ast: AST<'a>,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    ImportTerminalSign,
    InvalidIdentifier,
    ParamsCloseBracket,
    VarDefTerminal,
    LackOfBlockLeftBracket,
    LackOfBlockRightBracket,
    StructDefinitionTermial,
    UnionDefinitionTermial,
    LackOfMemberListLeftBracket,
    LackOfMemberListRightBracket,
    LackOfSlotTerminal,
    ExpectTypedef,
    TypedefTerminal,
    LackOfArrayCloseBracket,
    LackOfCloseParentheses,
    InvalidTyperefBase,
}


macro_rules! eat {
    ($Iter: expr) => ($Iter.next().unwrap());
    ($Iter: expr, $N: expr) => ($Iter.nth($N - 1).unwrap());
}

macro_rules! expect {
    ($Iter: expr, $Kind: ident else $Errorkind: ident) => ({
        lookahead!($Iter, if $Kind {
            eat!($Iter);
        }, else {
            return Err(ParseError::new(ParseErrorKind::$Errorkind));
        });
    });
    ($Iter: expr, $Kind: ident) => ({
        lookahead!($Iter, if $Kind {
            eat!($Iter);
        }, else {
            unreachable!()
        });
    })
}

macro_rules! lookahead {
    ($Iter: expr, if $Kind: ident $ThenBlock: block) => ({
        let mut scout = $Iter.clone();
        if let Some(&Token { kind: TokenKind::$Kind, .. }) =
                scout.next() {
            $ThenBlock;
        }
    });

    ($Iter: expr, if $Kind: ident $ThenBlock: block, else $ElseBlock: block) => ({
        let mut scout = $Iter.clone();
        if let Some(&Token { kind: TokenKind::$Kind, .. }) =
                scout.next() {
            $ThenBlock
        } else {
            $ElseBlock
        }
    });

    ($Iter: expr, $N: expr, if $Kind: ident $ThenBlock: block,
            else $ElseBlock: block) => ({
        let mut scout = $Iter.clone();
        if let Some(&Token { kind: TokenKind::$Kind, .. }) =
                scout.nth($N - 1) {
            $ThenBlock
        } else {
            $ElseBlock
        }
    });

    ($Iter: expr, while $Kind: ident $LoopBlock: block) => ({
        let mut scout = $Iter.clone();
        while let Some(&Token { kind: TokenKind::$Kind, .. }) =
                scout.next() {
            $LoopBlock;
            scout = $Iter.clone();
        }
    });

    ($Iter: expr, $($Kind: ident => $Block: block),+ else $ElseBlock: block) => ({
        let mut scout = $Iter.clone();
        match scout.next() {
            $(
                Some(&Token { kind: TokenKind::$Kind, .. }) => {
                    $Block
                },
            )+
            _ => $ElseBlock,
        }
    });
}

impl<'a> Parser<'a> {
    
    pub fn new(token_stream: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            iter: token_stream.iter(),
            ast: AST::new(),
        }
    }

    pub fn parse(&mut self) -> Result<AST> {
        self.syntax_analysis() ?;
        Ok(self.ast.clone())
    }

    fn syntax_analysis(&mut self) -> Result<()> {
        self.compilation_unit()
    }

    fn compilation_unit(&mut self) -> Result<()> {
        self.import_stmts() ?;
        self.top_defs() ?;
        self.eof()
    }

    fn import_stmts(&mut self) -> Result<()> {
        // let _stmts = Vec::new();

        lookahead!(self.iter, while Import { self.import_stmt() ?; });

        println!("Import Statements Finished!");

        Ok(())
    }

    fn top_defs(&mut self) -> Result<()> {
        lookahead!(self.iter, 
            Static => {
                self.defun_or_defvars()
            },
            Struct => {
                lookahead!(self.iter, 3, if LeftCurlyBracket {
                    self.defstruct()
                }, else {
                    self.defun_or_defvars()
                })
            },
            Union => {
                lookahead!(self.iter, 3, if LeftCurlyBracket {
                    self.defunion()
                }, else {
                    self.defun_or_defvars()
                })
            }
            else { unimplemented!() }
        )
    }

    fn defun_or_defvars(&mut self) -> Result<()> {
        lookahead!(self.iter, if Static {
            eat!(self.iter);
            // do someting...
        });

        self.typedef() ?;

        self.name() ?;

        lookahead!(self.iter, if OpenParentheses {
            eat!(self.iter);
            self.params() ?;
            expect!(self.iter, CloseParentheses else ParamsCloseBracket);
            self.block() ?;
            println!("Function Definition Found!");
            return Ok(());
        }, else {
            lookahead!(self.iter, if Equals {
                eat!(self.iter);
                self.expr() ?;
            });
            lookahead!(self.iter, while Comma {
                eat!(self.iter);
                self.name() ?;
                lookahead!(self.iter, if Equals {
                    eat!(self.iter);
                    self.expr() ?;
                });
            });
            expect!(self.iter, Semicolon else VarDefTerminal);
            println!("Variables Definition Found!");
            return Ok(());
        });
    }

    fn defstruct(&mut self) -> Result<()> {
        expect!(self.iter, Struct);

        self.name() ?;
        self.member_list() ?;

        expect!(self.iter, Semicolon else StructDefinitionTermial);

        println!("Structure definition Found!");
        Ok(())
    }

    fn defunion(&mut self) -> Result<()> {
        expect!(self.iter, Union);

        self.name() ?;
        self.member_list() ?;

        expect!(self.iter, Semicolon else UnionDefinitionTermial);

        println!("Union definition Found!");
        Ok(())
    }

    fn member_list(&mut self) -> Result<()> {
        expect!(self.iter, LeftCurlyBracket else LackOfMemberListLeftBracket);

        loop {
            lookahead!(self.iter, if RightCurlyBracket { break; });
            self.slot() ?;
            expect!(self.iter, Semicolon else LackOfSlotTerminal);
        }
        
        expect!(self.iter, RightCurlyBracket else LackOfMemberListRightBracket);

        println!("Member List Found!");
        Ok(())
    }

    fn slot(&mut self) -> Result<()> {
        self.type_() ?;
        self.name() ?;

        println!("Slot Found!");
        Ok(())
    }

    fn typedef(&mut self) -> Result<()> {
        expect!(self.iter, Typedef else ExpectTypedef);

        self.typeref() ?;
        self.name() ?; // Or Identifier?

        expect!(self.iter, Semicolon else TypedefTerminal);

        println!("Typedef Statement Found!");
        Ok(())
    }

    fn eof(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn import_stmt(&mut self) -> Result<()> {
        eat!(self.iter); // <Import>
        
        self.name() ?;

        lookahead!(self.iter, while Dot {
            eat!(self.iter); // <Dot>
            self.name() ?;
        });

        expect!(self.iter, Semicolon else ImportTerminalSign);
        println!("Import Statement Found!");
        Ok(())
    }

    fn name(&mut self) -> Result<()> {
        lookahead!(self.iter, if Identifier {
            eat!(self.iter); // <Identifier>
            println!("Identifier Found!");
            Ok(())
        }, else {
            println!("Identifier Error!");
            Err(ParseError::new(ParseErrorKind::InvalidIdentifier))
        })
    }

    fn params(&mut self) -> Result<()> {
        lookahead!(self.iter, if Void {
            lookahead!(self.iter, if CloseParentheses {
                eat!(self.iter, 2); // <Void> and ')'
                println!("Parameters with no element Found!");
                return Ok(());
            });
        });

        self.param() ?;

        lookahead!(self.iter, while Comma {
            eat!(self.iter); // ','
            lookahead!(self.iter, if Ellipsis {
                eat!(self.iter);
                println!("Variable parameter Found!");
                return Ok(());
            }, else {
                self.param() ?;
            });
        });

        println!("Fixed parameter list Found!");
        Ok(())
    }

    fn block(&mut self) -> Result<()> {
        expect!(self.iter, LeftCurlyBracket else LackOfBlockLeftBracket);
        self.defvar_list() ?;
        self.stmts() ?;
        expect!(self.iter, RightCurlyBracket else LackOfBlockRightBracket);
        println!("Block Found!");
        Ok(())
    }

    fn expr(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn param(&mut self) -> Result<()> {
        self.type_() ?;
        self.name() ?;
        println!("A Parameter Found");
        Ok(())
    }

    // crash the keyword `type`, so type_
    fn type_(&mut self) -> Result<()> {
        self.typeref() ?;
        println!("Type Found");
        Ok(())
    }

    fn typeref(&mut self) -> Result<()> {
        self.typeref_base() ?;
        loop {
            lookahead!(self.iter,
                ClosingBracket => {
                    eat!(self.iter); // '['
                    lookahead!(self.iter,
                        OpeningBracket => {
                            // Variable-length array
                        },
                        Integer => {
                            // Fixed-length array
                            eat!(self.iter); // <Integer>
                            expect!(self.iter, OpeningBracket else
                                LackOfArrayCloseBracket);
                        }
                        else {
                            return Err(ParseError::new(
                                ParseErrorKind::LackOfArrayCloseBracket));
                        }
                    )
                },
                Asterisk => {
                    eat!(self.iter); // '*'
                },
                OpenParentheses => {
                    self.param_typerefs() ?;
                    expect!(self.iter, CloseParentheses else 
                        LackOfCloseParentheses);
                }
                else { break; }
            );
        }

        println!("Typeref Found!");
        Ok(())
    }

    fn typeref_base(&mut self) -> Result<()> {
        lookahead!(self.iter,
            Void => {
                eat!(self.iter);
            },
            Char => {
                eat!(self.iter);
            },
            Short => {
                eat!(self.iter);
            },
            Int => {
                eat!(self.iter);
            },
            Long => {
                eat!(self.iter);
            },
            Unsigned => {
                eat!(self.iter); // <Unsigned>
                lookahead!(self.iter,
                    Char => {
                        eat!(self.iter); // <Char>
                    },
                    Short => {
                        eat!(self.iter); // <Short>
                    },
                    Int => {
                        eat!(self.iter); // <Int>
                    },
                    Long => {
                        eat!(self.iter); // <Long>
                    }
                    else { /* Just Unsigned */ }
                );
            },
            Struct => {
                eat!(self.iter);
                lookahead!(self.iter, if Identifier {
                    eat!(self.iter);
                }, else {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidIdentifier));
                });
            },
            Union => {
                eat!(self.iter);
                lookahead!(self.iter, if Identifier {
                    eat!(self.iter);
                }, else {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidIdentifier));
                });
            },
            Identifier => {
                eat!(self.iter); // p78?
            }
            else {
                return Err(ParseError::new(
                    ParseErrorKind::InvalidTyperefBase));
            }
        );

        println!("Typeref Base Found!");
        Ok(())
    }

    fn defvar_list(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn stmts(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn param_typerefs(&mut self) -> Result<()> {
        lookahead!(self.iter, if Void {
            lookahead!(self.iter, if CloseParentheses {
                eat!(self.iter, 2); // <Void> and ')'
                println!("Typedef parameters with no element Found!");
                return Ok(());
            });
        });

        self.typeref() ?;

        lookahead!(self.iter, while Comma {
            eat!(self.iter); // ','
            lookahead!(self.iter, if Ellipsis {
                eat!(self.iter);
                println!("Variable typeref parameters Found!");
                return Ok(());
            }, else {
                self.typeref() ?;
            });
        });

        println!("Fixed typeref parameter list Found!");
        Ok(())
    }
}


impl ParseError {
    fn new(kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind: kind,
        }
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ParseErrorKind::ImportTerminalSign => 
                "need a semicolon after the import sentence".fmt(f),
            ParseErrorKind::InvalidIdentifier =>
                "need a valid identifier".fmt(f),
            ParseErrorKind::ParamsCloseBracket =>
                "need a close bracket `)` for param list".fmt(f),
            ParseErrorKind::VarDefTerminal =>
                "need a semicolon after the variables definitions".fmt(f),
            ParseErrorKind::LackOfBlockLeftBracket =>
                "need a left curly bracket `{` for the block".fmt(f),
            ParseErrorKind::LackOfBlockRightBracket =>
                "need a right curly bracket `}` for the block".fmt(f),
            ParseErrorKind::StructDefinitionTermial =>
                "need a semicolon after the structure definition".fmt(f),
            ParseErrorKind::UnionDefinitionTermial =>
                "need a semicolon after the union definition".fmt(f),
            ParseErrorKind::LackOfMemberListLeftBracket =>
                "need a left curly bracket `{` before the Member List".fmt(f),
            ParseErrorKind::LackOfMemberListRightBracket =>
                "need a right curly bracket `}` before the Member List".fmt(f),
            ParseErrorKind::LackOfSlotTerminal =>
                "need a semicolon after the slot in member list".fmt(f),
            ParseErrorKind::ExpectTypedef =>
                "need a `typedef` keyword at the front of the typedef statement".fmt(f),
            ParseErrorKind::TypedefTerminal =>
                "need a semicolon after the typedef statement".fmt(f),
            ParseErrorKind::LackOfArrayCloseBracket =>
                "need a close bracket `]` in typeref".fmt(f),
            ParseErrorKind::LackOfCloseParentheses =>
                "need a close parentheses `)`".fmt(f),
            ParseErrorKind::InvalidTyperefBase =>
                "need a valid typeref base part".fmt(f),
        }
    }
}