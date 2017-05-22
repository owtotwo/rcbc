use super::token::{Token, TokenKind};
use super::ast::*;
use super::type_::*;
use super::location::Location;
use std::result;
use std::fmt;
use std::slice::Iter;
use std::mem;

type Result<'a, T> = result::Result<T, ParseError>;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
    ast: AST,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
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
    ExpressionTerminal,
    LackOfLeftBracketBeforeIfCond,
    LackOfRightBracketAfterIfCond,
    LackOfLeftBracketBeforeWhileCond,
    LackOfRightBracketAfterWhileCond,
    LackOfLeftBracketBeforeForCond,
    LackOfRightBracketAfterForCond,
    ForExpressionSeparator,
    LackOfLabel,
    ExpectWhileinDoWhile,
    DoWhileTerminal,
    LackOfLeftBracketBeforeSwitchCond,
    LackOfRightBracketAfterSwitchCond,
    LackOfLeftBracketBeforeCaseClause,
    LackOfRightBracketAfterCaseClause,
    BreakStatementTerminal,
    ExpectCaseColon,
    ExpectGotoLabel,
    GotoStatementTerminal,
    ReturnStatementTerminal,
    ContinueStatementTerminal,
    ExpectTernaryColon,
    ExpectCastRightBracket,
    ArrayReferenceTerminal,
    FunctionCallArgsTerminal,
    ExpectPrimaryRightBracket,
    InvalidPrimary,
}


macro_rules! eat {
    ($Iter: expr) => ($Iter.next().unwrap());
    ($Iter: expr, $N: expr) => ($Iter.nth($N - 1).unwrap());
}

macro_rules! expect {
    ($Iter: expr, $Kind: ident else $Errorkind: ident) => ({
        lookahead!($Iter, if $Kind {
            eat!($Iter)
        }, else {
            return Err(ParseError::new(ParseErrorKind::$Errorkind));
        })
    });
    ($Iter: expr, $Kind: ident) => ({
        lookahead!($Iter, if $Kind {
            eat!($Iter)
        }, else {
            unreachable!()
        })
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
        let location = token_stream
            .iter()
            .next()
            .map(|x| x.location())
            .unwrap_or(Location::default());
        Parser {
            iter: token_stream.iter(),
            ast: AST::new(location),
        }
    }

    pub fn parse(&mut self) -> Result<AST> {
        self.syntax_analysis()?;
        Ok(mem::replace(&mut self.ast, AST::new(Location::default())))
    }

    fn syntax_analysis(&mut self) -> Result<()> {
        self.compilation_unit()
    }

    fn compilation_unit(&mut self) -> Result<()> {
        self.import_stmts()?;
        self.top_defs()?;
        self.eof()
    }

    fn import_stmts(&mut self) -> Result<()> {
        // let _stmts = Vec::new();

        lookahead!(self.iter,
                   while Import {
                       self.import_stmt()?;
                   });

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
            },
            Typedef => {
                self.typedef()
            }
            else {
                self.defun_or_defvars()
            }
        )
    }

    fn defun_or_defvars(&mut self) -> Result<()> {
        lookahead!(self.iter,
                   if Static {
                       eat!(self.iter);
                       // do someting...
                   });

        self.typeref()?;

        self.name()?;

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

        self.name()?;
        self.member_list()?;

        expect!(self.iter, Semicolon else StructDefinitionTermial);

        println!("Structure definition Found!");
        Ok(())
    }

    fn defunion(&mut self) -> Result<()> {
        expect!(self.iter, Union);

        self.name()?;
        self.member_list()?;

        expect!(self.iter, Semicolon else UnionDefinitionTermial);

        println!("Union definition Found!");
        Ok(())
    }

    fn member_list(&mut self) -> Result<()> {
        expect!(self.iter, LeftCurlyBracket else LackOfMemberListLeftBracket);

        loop {
            lookahead!(self.iter,
                       if RightCurlyBracket {
                           break;
                       });
            self.slot()?;
            expect!(self.iter, Semicolon else LackOfSlotTerminal);
        }

        expect!(self.iter, RightCurlyBracket else LackOfMemberListRightBracket);

        println!("Member List Found!");
        Ok(())
    }

    fn slot(&mut self) -> Result<()> {
        self.type_()?;
        self.name()?;

        println!("Slot Found!");
        Ok(())
    }

    fn typedef(&mut self) -> Result<()> {
        expect!(self.iter, Typedef else ExpectTypedef);

        self.typeref()?;
        self.name()?; // Or Identifier?

        expect!(self.iter, Semicolon else TypedefTerminal);

        println!("Typedef Statement Found!");
        Ok(())
    }

    fn eof(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn import_stmt(&mut self) -> Result<()> {
        eat!(self.iter); // <Import>

        self.name()?;

        lookahead!(self.iter,
                   while Dot {
                       eat!(self.iter); // <Dot>
                       self.name()?;
                   });

        expect!(self.iter, Semicolon else ImportTerminalSign);
        println!("Import Statement Found!");
        Ok(())
    }

    fn name(&mut self) -> Result<Box<Node>> {
        lookahead!(self.iter, if Identifier {
            eat!(self.iter); // <Identifier>
            println!("Identifier Found!");
            // Ok(())
            unimplemented!()
        }, else {
            println!("Identifier Error!");
            Err(ParseError::new(ParseErrorKind::InvalidIdentifier))
        })
    }

    fn params(&mut self) -> Result<()> {
        lookahead!(self.iter,
                   if Void {
                       lookahead!(self.iter,
                                  if CloseParentheses {
                                      eat!(self.iter, 2); // <Void> and ')'
                                      println!("Parameters with no element Found!");
                                      return Ok(());
                                  });
                   });

        self.param()?;

        lookahead!(self.iter,
                   while Comma {
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
        self.defvar_list()?;
        self.stmts()?;
        expect!(self.iter, RightCurlyBracket else LackOfBlockRightBracket);
        println!("Block Found!");
        Ok(())
    }

    fn expr(&mut self) -> Result<Box<Node>> {
        let term = self.term()?;

        lookahead!(self.iter,
            Equals => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Assignment statement Found!");
                Ok(Box::new(AssignNode::new(location, lhs, rhs)))
            },
            AddAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Add assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::AddAssignment, rhs)))
            },
            SubtractAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Subtract assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::SubtractAssignment, rhs)))
            },
            MultiplyAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Multiply assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::MultiplyAssignment, rhs)))
            },
            DivideAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Divide assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::DivideAssignment, rhs)))
            },
            ModuloAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Modulo assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::ModuloAssignment, rhs)))
            },
            AndAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("And assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::AndAssignment, rhs)))
            },
            ExclusiveOrAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("ExclusiveOr assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::ExclusiveOrAssignment, rhs)))
            },
            OrAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("Or assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::OrAssignment, rhs)))
            },
            LeftShiftAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("LeftShift assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::LeftShiftAssignment, rhs)))
            },
            RightShiftAssign => {
                let lhs = term;
                eat!(self.iter);
                let rhs = self.expr() ?;
                let location = Location::range(lhs.location(), rhs.location());
                println!("RightShift assignment statement Found!");
                Ok(Box::new(OpAssignNode::new(location, lhs, OpAssignType::RightShiftAssignment, rhs)))
            }
            else {
                let expr = self.expr_10(Some(term)) ?;

                println!("Expression Found!");
                Ok(expr)
            }
        )
    }

    fn expr_10(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let condition = self.expr_9(term)?;

        lookahead!(self.iter,
                   if QuestionMark {
            let then_clause = self.expr()?;
            expect!(self.iter, Colon else ExpectTernaryColon);
            let else_clause = self.expr_10(None)?;
            let location = Location::range(condition.location(), else_clause.location());
            return Ok(Box::new(CondExprNode::new(location, condition, then_clause, else_clause)));
        });

        Ok(condition)
    }

    fn expr_9(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_8(term)?;

        lookahead!(self.iter,
                   while LogicalOr {
                       eat!(self.iter);
                       let right = self.expr_8(None)?;
                       let location = Location::range(left.location(), right.location());
                       left = Box::new(LogicalOrNode::new(location, left, right));
                   });

        Ok(left)
    }

    fn expr_8(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_7(term)?;

        lookahead!(self.iter,
                   while LogicalAnd {
                       eat!(self.iter);
                       let right = self.expr_7(None)?;
                       let location = Location::range(left.location(), right.location());
                       left = Box::new(LogicalAndNode::new(location, left, right));
                   });

        Ok(left)
    }

    fn expr_7(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_6(term)?;

        loop {
            lookahead!(self.iter,
                GreaterThan => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::GreaterThan, right));
                },
                LessThan => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::LessThan, right));
                },
                DoubleEquals => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::DoubleEquals, right));
                },
                NotEqualTo => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::NotEqualTo, right));
                },
                LessThanOrEqualTo => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::LessThanOrEqualTo, right));
                },
                GreaterThanOrEqualTo => {
                    eat!(self.iter);
                    let right = self.expr_6(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::GreaterThanOrEqualTo, right));
                }
                else { break; }
            );
        }

        Ok(left)
    }

    fn expr_6(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_5(term)?;

        lookahead!(self.iter,
                   while VerticalBar {
                       eat!(self.iter);
                       let right = self.expr_5(None)?;
                       let location = Location::range(left.location(), right.location());
                       left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::BitOr, right));
                   });

        Ok(left)
    }

    fn expr_5(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_4(term)?;

        lookahead!(self.iter,
                   while Caret {
                       eat!(self.iter);
                       let right = self.expr_4(None)?;
                       let location = Location::range(left.location(), right.location());
                       left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::BitExclusiveOr, right));
                   });

        Ok(left)
    }

    fn expr_4(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_3(term)?;

        lookahead!(self.iter,
                   while Ampersand {
                       eat!(self.iter);
                       let right = self.expr_3(None)?;
                       let location = Location::range(left.location(), right.location());
                       left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::BitAnd, right));
                   });

        Ok(left)
    }

    fn expr_3(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_2(term)?;

        loop {
            lookahead!(self.iter,
                LeftShift => {
                    eat!(self.iter);
                    let right = self.expr_2(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::LeftShift, right));
                },
                RightShift => {
                    eat!(self.iter);
                    let right = self.expr_2(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::RightShift, right));
                }
                else { break; }
            );
        }

        Ok(left)
    }

    fn expr_2(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = self.expr_1(term)?;

        loop {
            lookahead!(self.iter,
                Plus => {
                    eat!(self.iter);
                    let right = self.expr_1(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::Addition, right));
                },
                Hyphen => {
                    eat!(self.iter);
                    let right = self.expr_1(None) ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::Subtraction, right));
                }
                else { break; }
            );
        }

        Ok(left)
    }

    fn expr_1(&mut self, term: Option<Box<Node>>) -> Result<Box<Node>> {
        let mut left = if term.is_some() {
            term.unwrap()
        } else {
            self.term() ?
        };

        loop {
            lookahead!(self.iter,
                Asterisk => {
                    eat!(self.iter);
                    let right = self.term() ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::Multiplication, right));
                },
                Slash => {
                    eat!(self.iter);
                    let right = self.term() ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::Division, right));
                },
                Procenttecken => {
                    eat!(self.iter);
                    let right = self.term() ?;
                    let location = Location::range(left.location(), right.location());
                    left = Box::new(BinaryOpNode::new(location, left, BinaryOpType::Modulo, right));
                }
                else { break; }
            );
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Box<Node>> {
        lookahead!(self.iter, if OpenParentheses {
            let open = eat!(self.iter);
            match self.type_() { // just try
                Ok(type_) => {
                    let close: &Token = expect!(self.iter, CloseParentheses else
                        ExpectCastRightBracket);
                    let node = self.term() ?;
                    println!("Casting Term Found!");
                    let location = Location::range(open.location(), close.location());
                    Ok(Box::new(CastNode::new(location, type_, node)))
                },
                Err(ParseError { kind: ParseErrorKind::InvalidTyperefBase }) => {
                    let node = self.unary(true) ?;
                    println!("Unary Term Found!");
                    Ok(node) // TODO: should update location
                },
                Err(e) => {
                    Err(e) // real error
                }
            }
        }, else {
            let node = self.unary(false) ?;
            println!("Unary Term Found!");
            Ok(node)
        })
    }

    fn unary(&mut self, has_ate_left_bracket: bool) -> Result<Box<Node>> {
        if has_ate_left_bracket {
            let node = self.postfix(has_ate_left_bracket)?;
            println!("Unary Found!");
            return Ok(node);
        }

        lookahead!(self.iter,
            Increment => {
                let token: &Token = eat!(self.iter);
                let node = self.unary(false) ?;
                return Ok(Box::new(
                    PrefixOpNode::new(token.location(), PrefixOpType::Increment, node)
                ));
            },
            Decrement => {
                let token: &Token = eat!(self.iter);
                let node = self.unary(false) ?;
                return Ok(Box::new(
                    PrefixOpNode::new(token.location(), PrefixOpType::Decrement, node)
                ));
            },
            Plus => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    UnaryOpNode::new(token.location(), UnaryOpType::Plus, node)
                ));
            },
            Hyphen => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    UnaryOpNode::new(token.location(), UnaryOpType::Hyphen, node)
                ));
            },
            ExclamationMark => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    UnaryOpNode::new(token.location(), UnaryOpType::ExclamationMark, node)
                ));
            },
            Tilde => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    UnaryOpNode::new(token.location(), UnaryOpType::Tilde, node)
                ));
            },
            Asterisk => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    DereferenceNode::new(token.location(), node)
                ));
            },
            Ampersand => {
                let token: &Token = eat!(self.iter);
                let node = self.term() ?;
                return Ok(Box::new(
                    AddressNode::new(token.location(), node)
                ));
            },
            Sizeof => {
                let left = eat!(self.iter);
                lookahead!(self.iter, if OpenParentheses {
                    eat!(self.iter);
                    match self.type_() { // just try
                        Ok(type_) => {
                            let right = expect!(self.iter, CloseParentheses else ExpectCastRightBracket);
                            println!("Sizeof(type) Found!");
                            let location = Location::range(left.location(), right.location());
                            return Ok(Box::new(SizeofTypeNode::new(location, type_, unimplemented!())));
                        },
                        Err(ParseError { kind: ParseErrorKind::InvalidTyperefBase }) => {
                            let node = self.unary(true) ?;
                            println!("Sizeof expression Found!");
                            let location = Location::range(left.location(), node.location());
                            return Ok(Box::new(SizeofExprNode::new(location, node, unimplemented!())));
                        },
                        Err(e) => {
                            return Err(e); // real error
                        }
                    };
                }, else {
                    let node = self.unary(false) ?;
                    println!("Sizeof expression Found!");
                    let location = Location::range(left.location(), node.location());
                    return Ok(Box::new(SizeofExprNode::new(location, node, unimplemented!())));
                });
            }
            else {
                let node = self.postfix(false) ?;
                return Ok(node)
            }
        );

        println!("Unary Found!");
    }

    fn postfix(&mut self, has_ate_left_bracket: bool) -> Result<Box<Node>> {
        let mut expr = self.primary(has_ate_left_bracket) ?;
        loop {
            lookahead!(self.iter,
                Increment => {
                    let right = eat!(self.iter);
                    let location = Location::range(expr.location(), right.location());
                    expr = Box::new(SuffixOpNode::new(location, SuffixOpType::Increment, expr));
                },
                Decrement => {
                    let right = eat!(self.iter);
                    let location = Location::range(expr.location(), right.location());
                    expr = Box::new(SuffixOpNode::new(location, SuffixOpType::Decrement, expr));
                },
                OpeningBracket => {
                    eat!(self.iter);
                    let idx = self.expr() ?;
                    let right = expect!(self.iter, ClosingBracket else ArrayReferenceTerminal);
                    println!("Array reference postfix Found!");
                    let location = Location::range(expr.location(), right.location());
                    expr = Box::new(ArefNode::new(location, expr, idx));
                },
                Dot => {
                    eat!(self.iter);
                    let memb = self.name() ?;
                    println!("Structure or Union member reference postfix Found!");
                    let location = Location::range(expr.location(), memb.location());
                    expr = Box::new(MemberNode::new(location, expr, memb));
                },
                Arrow => {
                    eat!(self.iter);
                    let memb = self.name() ?;
                    println!("Reference by pointer postfix Found!");
                    let location = Location::range(expr.location(), memb.location());
                    expr = Box::new(PtrMemberNode::new(location, expr, memb));
                },
                OpenParentheses => {
                    eat!(self.iter);
                    let args: Vec<Box<Node>> = self.args() ?;
                    let right = expect!(self.iter, CloseParentheses else FunctionCallArgsTerminal);
                    let location = Location::range(expr.location(), right.location());
                    println!("Function call postfix Found!");
                    expr = Box::new(FuncallNode::new(location, expr, args));
                }
                else { break; }
            );
        }

        println!("postfix Found!");
        Ok(expr)
    }

    fn args(&mut self) -> Result<Vec<Box<Node>>> {
        lookahead!(self.iter, if CloseParentheses { /* Empty args */ }, else {
            self.expr() ?;
            lookahead!(self.iter, while Comma {
                eat!(self.iter);
                self.expr() ?;
            });
        });

        println!("Args Found!");
        // Ok(())
        unimplemented!()
    }

    fn param(&mut self) -> Result<()> {
        self.type_()?;
        self.name()?;
        println!("A Parameter Found!");
        Ok(())
    }

    // crash the keyword `type`, so type_
    fn type_(&mut self) -> Result<Box<TypeRef>> {
        let typeref = self.typeref()?;
        println!("Type Found!");
        Ok(typeref)
    }

    fn typeref(&mut self) -> Result<Box<TypeRef>> {
        self.typeref_base()?;
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
        unimplemented!()
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
                if !self.is_type(self.iter.clone().next().unwrap().image()) {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidTyperefBase));
                }
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
        loop {
            lookahead!(self.iter,
                       if Static {
                           eat!(self.iter);
                           // do someting...
                       });

            match self.typeref() {
                Ok(_) => { /* is variable definition list */ }
                Err(ParseError { kind: ParseErrorKind::InvalidTyperefBase }) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            };

            self.name()?;

            lookahead!(self.iter,
                       if Equals {
                           eat!(self.iter);
                           self.expr()?;
                       });

            lookahead!(self.iter,
                       while Comma {
                           eat!(self.iter);
                           self.name()?;
                           lookahead!(self.iter,
                                      if Equals {
                                          eat!(self.iter);
                                          self.expr()?;
                                      });
                       });

            expect!(self.iter, Semicolon else VarDefTerminal);
        }

        println!("Variables Definition in function Found!");
        Ok(())
    }

    fn stmts(&mut self) -> Result<()> {
        loop {
            lookahead!(self.iter,
                       if RightCurlyBracket {
                           break;
                       });
            self.stmt()?;
        }

        println!("Statements Found!");
        Ok(())
    }

    fn stmt(&mut self) -> Result<Box<Node>> {
        lookahead!(self.iter,
            Semicolon => {
                eat!(self.iter);
            },
            LeftCurlyBracket => { // block
                self.block() ?;
            },
            If => {
                self.if_stmt() ?;
            },
            While => {
                self.while_stmt() ?;
            },
            Do => {
                self.dowhile_stmt() ?;
            },
            For => {
                self.for_stmt() ?;
            },
            Switch => {
                self.switch_stmt() ?;
            },
            Break => {
                self.break_stmt() ?;
            },
            Continue => {
                self.continue_stmt() ?;
            },
            Goto => {
                self.goto_stmt() ?;
            },
            Return => {
                self.return_stmt() ?;
            },
            Identifier => {
                lookahead!(self.iter, 2, if Colon {
                    self.labeled_stmt() ?;
                }, else {
                    self.expr() ?;
                    expect!(self.iter, Semicolon else ExpressionTerminal);
                });
            }
            else {
                self.expr() ?;
                expect!(self.iter, Semicolon else ExpressionTerminal);
            }
        );

        println!("Statment Found!");
        unimplemented!()
    }

    fn if_stmt(&mut self) -> Result<Box<Node>> {
        let if_token = expect!(self.iter, If);
        expect!(self.iter, OpenParentheses else LackOfLeftBracketBeforeIfCond);
        let condition = self.expr() ?;
        expect!(self.iter, CloseParentheses else LackOfRightBracketAfterIfCond);
        let then_clause = self.stmt() ?;
        lookahead!(self.iter,
                   if Else {
                       eat!(self.iter);
                       let else_clause = self.stmt() ?;
                       let location = Location::range(if_token.location(), else_clause.location());
                       return Ok(Box::new(IfNode::new(location, condition, then_clause, Some(else_clause))));
                   });

        println!("If statement Found!");
        let location = Location::range(if_token.location(), then_clause.location());
        Ok(Box::new(IfNode::new(location, condition, then_clause, None)))
    }

    fn while_stmt(&mut self) -> Result<Box<Node>> {
        let while_token = expect!(self.iter, While);
        expect!(self.iter, OpenParentheses else LackOfLeftBracketBeforeWhileCond);
        let condition = self.expr() ?;
        expect!(self.iter, CloseParentheses else LackOfRightBracketAfterWhileCond);
        let body = self.stmt() ?;

        println!("While statement Found!");
        let location = Location::range(while_token.location(), body.location());
        Ok(Box::new(WhileNode::new(location, condition, body)))
    }

    fn dowhile_stmt(&mut self) -> Result<Box<Node>> {
        let do_token = expect!(self.iter, Do);
        let body = self.stmt() ?;
        expect!(self.iter, While else ExpectWhileinDoWhile);
        expect!(self.iter, OpenParentheses else LackOfLeftBracketBeforeWhileCond);
        let condition = self.expr()?;
        expect!(self.iter, CloseParentheses else LackOfRightBracketAfterWhileCond);
        expect!(self.iter, Semicolon else DoWhileTerminal);

        println!("Do-While statement Found!");
        let location = Location::range(do_token.location(), condition.location());
        Ok(Box::new(DoWhileNode::new(location, body, condition)))
    }

    fn for_stmt(&mut self) -> Result<Box<Node>> {
        let for_token = expect!(self.iter, For);
        expect!(self.iter, OpenParentheses else LackOfLeftBracketBeforeForCond);
        let init_expr = lookahead!(self.iter, if Semicolon { None /* do nothing */ }, else {
            Some(self.expr() ?)
        });
        expect!(self.iter, Semicolon else ForExpressionSeparator);
        let cond_expr = lookahead!(self.iter, if Semicolon { None /* do nothing */ }, else {
            Some(self.expr() ?)
        });
        expect!(self.iter, Semicolon else ForExpressionSeparator);
        let step_expr = lookahead!(self.iter, if CloseParentheses { None /* do nothing */ }, else {
            Some(self.expr() ?)
        });
        expect!(self.iter, CloseParentheses else LackOfRightBracketAfterForCond);
        let body = self.stmt()?;

        println!("For statement Found!");
        let location = Location::range(for_token.location(), body.location());
        Ok(Box::new(ForNode::new(location, init_expr, cond_expr, step_expr, body)))
    }

    fn switch_stmt(&mut self) -> Result<Box<Node>> {
        let switch_token = expect!(self.iter, Switch);
        expect!(self.iter, OpenParentheses else LackOfLeftBracketBeforeSwitchCond);
        let expr = self.expr() ?;
        expect!(self.iter, CloseParentheses else LackOfRightBracketAfterSwitchCond);
        expect!(self.iter, LeftCurlyBracket else LackOfLeftBracketBeforeCaseClause);
        let cases = self.case_clauses() ?;
        let close_token = expect!(self.iter, RightCurlyBracket else LackOfRightBracketAfterCaseClause);

        println!("Switch statement Found!");
        let location = Location::range(switch_token.location(), close_token.location());
        Ok(Box::new(SwitchNode::new(location, expr, cases)))
    }

    fn break_stmt(&mut self) -> Result<Box<Node>> {
        let break_token = expect!(self.iter, Break);
        let semicolon_token = expect!(self.iter, Semicolon else BreakStatementTerminal);

        println!("Break statement Found!");
        let location = Location::range(break_token.location(), semicolon_token.location());
        Ok(Box::new(BreakNode::new(location)))
    }

    fn continue_stmt(&mut self) -> Result<Box<Node>> {
        let continue_token = expect!(self.iter, Continue);
        let semicolon_token = expect!(self.iter, Semicolon else ContinueStatementTerminal);

        println!("Continue statement Found!");
        let location = Location::range(continue_token.location(), semicolon_token.location());
        Ok(Box::new(ContinueNode::new(location)))
    }

    fn goto_stmt(&mut self) -> Result<Box<Node>> {
        let goto_token = expect!(self.iter, Goto);
        let label = lookahead!(self.iter, if Identifier {
            eat!(self.iter)
        }, else {
            return Err(ParseError::new(ParseErrorKind::ExpectGotoLabel));
        });
        let semicolon_token = expect!(self.iter, Semicolon else GotoStatementTerminal);

        println!("Goto statement Found!");
        let location = Location::range(goto_token.location(), semicolon_token.location());
        Ok(Box::new(GotoNode::new(location, label.value().unwrap())))
    }

    fn return_stmt(&mut self) -> Result<Box<Node>> {
        let return_token = expect!(self.iter, Return);
        let expr = lookahead!(self.iter, if Semicolon { None /* no return value */ }, else {
            // have return value
            Some(self.expr() ?)
        });
        let semicolon_token = expect!(self.iter, Semicolon else ReturnStatementTerminal);

        println!("Return statement Found!");
        let location = Location::range(return_token.location(), semicolon_token.location());
        Ok(Box::new(ReturnNode::new(location, expr)))
    }

    fn labeled_stmt(&mut self) -> Result<Box<Node>> {
        let label = lookahead!(self.iter, if Identifier {
            eat!(self.iter)
        }, else {
            return Err(ParseError::new(ParseErrorKind::LackOfLabel));
        });
        expect!(self.iter, Colon);
        let stmt = self.stmt() ?;

        println!("Labeled statement Found!");
        let location = Location::range(label.location(), stmt.location());
        Ok(Box::new(LabelNode::new(location, label.value().unwrap(), stmt)))
    }

    fn case_clauses(&mut self) -> Result<Option<Box<Node>>> {
        let mut normal_cases = Vec::new();
        lookahead!(self.iter,
                   while Case {
                       normal_cases.push(self.case_clause() ?);
                   });

        let mut default_case = None;
        lookahead!(self.iter,
                   if Default {
                       default_case = Some(self.default_clause() ?);
                   });

        println!("Case clauses Found!");
        if normal_cases.len() == 0 && default_case.is_none() {
            Ok(None)
        } else {
            let location = if let Some(ref case) = default_case {
                if normal_cases.len() > 0 {
                    Location::range(normal_cases[0].location(), case.location())
                } else {
                    case.location()
                }
            } else {
                Location::range(normal_cases[0].location(), normal_cases[normal_cases.len() - 1].location())
            };
            Ok(Some(Box::new(CasesNode::new(location, normal_cases, default_case))))
        }
    }

    fn case_clause(&mut self) -> Result<Box<Node>> {
        let expr = self.case() ?;
        let stmts = self.case_body() ?;

        println!("Case clause Found!");
        let right = if stmts.len() > 0 {
            stmts[stmts.len() - 1].location()
        } else {
            expr.location()
        };
        let location = Location::range(expr.location(), right);
        Ok(Box::new(CaseNode::new(location, expr, stmts)))
    }

    fn default_clause(&mut self) -> Result<Box<Node>> {
        let default_token = expect!(self.iter, Default);
        let colon_token = expect!(self.iter, Colon else ExpectCaseColon);
        let stmts = self.case_body() ?;
        println!("Default clause Found!");
        let right = if stmts.len() > 0 {
            stmts[stmts.len() - 1].location()
        } else {
            colon_token.location()
        };
        let location = Location::range(default_token.location(), right);
        Ok(Box::new(DefaultCaseNode::new(location, stmts)))
    }

    fn case(&mut self) -> Result<Box<Node>> {
        let case_token = expect!(self.iter, Case);
        let expr = self.primary(false) ?;
        expect!(self.iter, Colon else ExpectCaseColon);

        println!("Case head Found!");
        Ok(expr)
    }

    fn case_body(&mut self) -> Result<Vec<Box<Node>>> {
        let mut stmts = Vec::new();
        loop {
            stmts.push(self.stmt() ?);
            lookahead!(self.iter,
                Case => { break; },
                Default => { break; },
                RightCurlyBracket => { break; }
                else { /* continue to get the stmt */ }
            );
        }

        println!("Case body Found!");
        Ok(stmts)
    }

    fn param_typerefs(&mut self) -> Result<()> {
        lookahead!(self.iter,
                   if Void {
                       lookahead!(self.iter,
                                  if CloseParentheses {
                                      eat!(self.iter, 2); // <Void> and ')'
                                      println!("Typedef parameters with no element Found!");
                                      return Ok(());
                                  });
                   });

        self.typeref()?;

        lookahead!(self.iter,
                   while Comma {
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

    fn primary(&mut self, has_ate_left_bracket: bool) -> Result<Box<Node>> {
        if has_ate_left_bracket {
            self.expr()?;
            expect!(self.iter, CloseParentheses else
                ExpectPrimaryRightBracket);
        } else {
            lookahead!(self.iter,
                Integer => {
                    let token: &Token = eat!(self.iter);
                    return Ok(Box::new(helper::integer_node(token.location(), token.value().unwrap())))
                },
                Character => {
                    let token: &Token = eat!(self.iter);
                    return Ok(Box::new(
                        IntegerLiteralNode::new(
                            token.location(),
                            IntegerTypeRef::Char,
                            helper::character_code(token.value().unwrap())
                        )
                    ))
                },
                String => {
                    let token: &Token = eat!(self.iter);
                    return Ok(Box::new(
                        StringLiteralNode::new(token.location(), token.value().unwrap())
                    ))
                },
                Identifier => {
                    let token: &Token = eat!(self.iter);
                    return Ok(Box::new(
                        VariableNode::new(token.location(), token.value().unwrap())
                    ))
                },
                OpenParentheses => {
                    eat!(self.iter);
                    let node = self.expr() ?;
                    expect!(self.iter, CloseParentheses else
                        ExpectPrimaryRightBracket);
                    // return Ok(Box::new(node))
                    unimplemented!()
                }
                else {
                    return Err(ParseError::new(ParseErrorKind::InvalidPrimary));
                }
            );
        }

        println!("Primary Found!");
        unimplemented!()
    }

    fn is_type(&self, name: String) -> bool {
        false // unimplemented!()
    }
}


impl ParseError {
    fn new(kind: ParseErrorKind) -> ParseError {
        ParseError { kind: kind }
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
            ParseErrorKind::ExpressionTerminal =>
                "need a semicolon after the expression statement".fmt(f),
            ParseErrorKind::LackOfLeftBracketBeforeIfCond =>
                "need a open parentheses before the `if` condition".fmt(f),
            ParseErrorKind::LackOfRightBracketAfterIfCond =>
                "need a close parentheses after the `if` condition".fmt(f),
            ParseErrorKind::LackOfLeftBracketBeforeWhileCond =>
                "need a open parentheses before the `while` condition".fmt(f),
            ParseErrorKind::LackOfRightBracketAfterWhileCond =>
                "need a close parentheses after the `while` condition".fmt(f),
            ParseErrorKind::LackOfLeftBracketBeforeForCond =>
                "need a open parentheses before the `for` condition".fmt(f),
            ParseErrorKind::LackOfRightBracketAfterForCond =>
                "need a close parentheses after the `for` condition".fmt(f),
            ParseErrorKind::ForExpressionSeparator =>
                "need a semicolon as the separator between \
                 expressions in `for` statement".fmt(f),
            ParseErrorKind::LackOfLabel =>
                "need a identifier as a label in labeled statment".fmt(f),
            ParseErrorKind::ExpectWhileinDoWhile =>
                "need the `while` in do-while statement".fmt(f),
            ParseErrorKind::DoWhileTerminal =>
                "need a semicolon after the do-while statement".fmt(f),
            ParseErrorKind::LackOfLeftBracketBeforeSwitchCond =>
                "need a open parentheses before the `switch` condition".fmt(f),
            ParseErrorKind::LackOfRightBracketAfterSwitchCond =>
                "need a close parentheses after the `switch` condition".fmt(f),
            ParseErrorKind::LackOfLeftBracketBeforeCaseClause =>
                "need a left curly bracket before case clauses \
                 in `switch` statement".fmt(f),
            ParseErrorKind::LackOfRightBracketAfterCaseClause =>
                "need a right curly bracket after case clauses \
                 in `switch` statement".fmt(f),
            ParseErrorKind::BreakStatementTerminal =>
                "need a semicolon after the break statement".fmt(f),
            ParseErrorKind::ExpectCaseColon =>
                "need a colon after the case value".fmt(f),
            ParseErrorKind::ExpectGotoLabel =>
                "need a label after the `goto` keyword".fmt(f),
            ParseErrorKind::GotoStatementTerminal =>
                "need a semicolon after the goto statement".fmt(f),
            ParseErrorKind::ReturnStatementTerminal =>
                "need a semicolon after the return statement".fmt(f),
            ParseErrorKind::ContinueStatementTerminal =>
                "need a semicolon after the continue statement".fmt(f),
            ParseErrorKind::ExpectTernaryColon =>
                "need a colon between the expressions in ternary \
                 expression".fmt(f),
            ParseErrorKind::ExpectCastRightBracket =>
                "need a close parentheses for the cast type".fmt(f),
            ParseErrorKind::ArrayReferenceTerminal =>
                "need a close bracket `]` in array reference".fmt(f),
            ParseErrorKind::FunctionCallArgsTerminal =>
                "need a close parentheses after the function call \
                 argument list".fmt(f),
            ParseErrorKind::ExpectPrimaryRightBracket =>
                "need a close parentheses after the expression".fmt(f),
            ParseErrorKind::InvalidPrimary => // what?
                "need a valid primary".fmt(f),
        }
    }
}

