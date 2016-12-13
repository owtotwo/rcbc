use super::location::Location;
use super::type_::*;

const INDENT_STRING: &'static str = "    ";

trait Node {
    fn location(&self) -> &Location;
    fn dump(&self, indent_level: usize);
}

macro_rules! impl_node_location {
    ($T: ty) => (
        impl Node for $T {
            fn location(&self) -> &Location {
                &self.location
            }
            fn dump(&self, indent_level: usize) {
                let indents: Vec<&'static str> = (0..indent_level)
                                                    .map(|_| INDENT_STRING)
                                                    .collect();
                let indent: String = indents.concat();
                println!("{}{}", indent, "<Content...>");
            }
        }
    )
}


trait ExprNode: Node {
    // ...
}

trait AssignNodeTrait: ExprNode {
    // ...
}

pub struct AssignNode {
    // ...
}

pub struct OpAssignNode {
    // ...
}

pub struct AddressNode {
    // ...
}

trait BinaryOpTrait: ExprNode {
    // ...
}

pub struct BinaryOpNode {
    location: Location,
    operator: String,
    left: Box<ExprNode>,
    right: Box<ExprNode>,
    // type_: Box<Type>,
}

impl_node_location!(BinaryOpNode);

impl BinaryOpNode {
    fn new(left: Box<ExprNode>, op: String, right: Box<ExprNode>) -> BinaryOpNode {
        BinaryOpNode {
            location: Location {
                begin: left.location().clone().begin,
                end: right.location().clone().end,
            },
            left: left,
            operator: op,
            right: right,
        }
    }    
}

pub struct LogicalAndNode {
    // ...
}

pub struct LogicalOrNode {
    // ...
}

pub struct CastNode {
    // ...
}

pub struct CondExprNode {
    // ...
}

pub struct FuncallNode {
    // ...
}

trait LHSNode: ExprNode {
    // ...
}

pub struct ArefNode {
    // ...
}

pub struct DereferenceNode {
    // ...
}

pub struct MemberNode {
    // ...
}

pub struct PtrMemberNode {
    // ...
}

pub struct VariableNode {
    location: Location,
    name: String,
    // entity: Entity,
}

trait LiteralNode: ExprNode {
    // ...
}

pub struct IntegerLiteralNode {
    location: Location,
    type_node: Box<TypeNode>,
    value: i64,
}

pub struct StringLiteralNode {
    // ...
}

pub struct SizeofExprNode {
    // ...
}

pub struct SizeofTypeNode {
    // ...
}

trait UnaryOpNode: ExprNode {
    // ...
}

trait UnaryArithmeticOpNode: UnaryOpNode {
    // ...
}

pub struct PrefixOpNode {
    // ...
}

pub struct SuffixOpNode {
    // ...
}

pub struct Slot {
    // ...
}

trait StmtNode: Node {
    // ...
}

pub struct BlockNode {
    // ...
}

pub struct BreakNode {
    // ...
}

pub struct CaseNode {
    // ...
}

pub struct ContinueNode {
    // ...
}

pub struct DoWhileNode {
    // ...
}

pub struct ExprStmtNode {
    // ...
}

pub struct ForNode {
    // ...
}

pub struct GotoNode {
    // ...
}

pub struct IfNode {
    // ...
}

pub struct LabelNode {
    // ...
}

pub struct ReturnNode {
    // ...
}

pub struct SwitchNode {
    // ...
}

pub struct WhileNode {
    // ...
}

trait TypeDefinition: Node {
    // ...
}

trait CompositeTypeDefinition: TypeDefinition {
    // ...
}

pub struct StructNode {
    // ...
}

pub struct UnionNode {
    // ...
}

trait TypedefNode: TypeDefinition {
    // ...
}

pub enum TypeNode {
    Type(Box<Type>),
    TypeRef(Box<TypeRef>),
}


#[derive(Debug, Clone)]
pub struct AST {
    location: Location,
    // declarations: Declarations,
}

impl AST {
    pub fn new(location: Location) -> AST {
        AST {
            location: location,
        }
    }
}

impl Node for AST {
    fn location(&self) -> &Location {
        &self.location
    }

    fn dump(&self, indent_level: usize) {

    }
}

impl IntegerLiteralNode {
    fn new(loc: Location, typeref: Box<TypeRef>, val: i64) -> IntegerLiteralNode {
        IntegerLiteralNode {
            location: loc,
            type_node: Box::new(TypeNode::new(typeref)),
            value: val,
        }
    }
}

impl TypeNode {
    fn new(typeref: Box<TypeRef>) -> TypeNode {
        TypeNode::TypeRef(typeref)
    }
}

// #[derive(Debug, Clone)]
// pub struct Declarations<'a> {
//     declarations: Vec<Declaration<'a>>,
// }

// #[derive(Debug, Clone)]
// pub enum Declaration<'a> {
//     VarDef(DefinedVariable<'a>),
//     VarDecl(UndefinedVariable<'a>),
//     FuncDef(DefinedFunction<'a>),
//     FuncDecl(UndefinedFunction<'a>),
//     Const(Constant<'a>),
//     StructDef(StructNode<'a>),
//     UnionDef(UnionNode<'a>),
//     TypeDef(TypedefNode<'a>),
// }


// #[derive(Debug, Clone)]
// pub struct UndefinedVariable<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     is_private: bool,
// }

// #[derive(Debug, Clone)]
// pub struct DefinedVariable<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     is_private: bool,
//     initializer: ExprNode,
// }

// #[derive(Debug, Clone)]
// pub struct DefinedFunction<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     is_private: bool,
//     params: Params<'a>,
//     body: BlockNode<'a>,
// }

// #[derive(Debug, Clone)]
// pub struct UndefinedFunction<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     is_private: bool,
//     params: Params<'a>,
// }

// #[derive(Debug, Clone)]
// pub struct Constant<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     value: ExprNode,
// }

// #[derive(Debug, Clone)]
// pub struct StructNode<'a> {
//     location: Location<'a>,
//     type_ref: TypeRef<'a>,
//     name: String,
//     membs: Vec<Slot<'a>>,
// }

// #[derive(Debug, Clone)]
// pub struct UnionNode<'a> {
//     location: Location<'a>,
//     type_ref: TypeRef<'a>,
//     name: String,
//     membs: Vec<Slot<'a>>,
// }

// #[derive(Debug,  Clone)]
// pub struct TypedefNode<'a> {
//     location: Location<'a>,
//     name: String,
//     real: TypeNode<'a>,
// }


// #[derive(Debug, Clone)]
// pub struct TypeNode<'a> {
//     type_ref: TypeRef<'a>,
//     self_type: Type,
// }

// #[derive(Debug,  Clone)]
// pub struct TypeRef<'a> {
//     location: Location<'a>,
// }

// #[derive(Debug,  Clone)]
// pub struct Type {
//     // ...
// }


// #[derive(Debug,  Clone)]
// pub struct ExprNode {
//     // ...
// }

// #[derive(Debug,  Clone)]
// pub struct Params<'a> {
//     location: Location<'a>,
//     param_descs: Vec<Parameter<'a>>,
// }

// #[derive(Debug,  Clone)]
// pub struct BlockNode<'a> {
//     location: Location<'a>,
//     variables: Vec<DefinedVariable<'a>>,
//     stmts: Vec<StmtNode<'a>>,
// }


// #[derive(Debug,  Clone)]
// pub struct Slot<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
//     offset: usize,
// }

// #[derive(Debug,  Clone)]
// pub struct Parameter<'a> {
//     type_node: TypeNode<'a>,
//     name: String,
// }

// #[derive(Debug,  Clone)]
// pub struct StmtNode<'a> {
//     location: Location<'a>,
// }



// impl<'a> AST<'a> {
//     pub fn new() -> AST<'a> {
//         AST {
//             declarations: Declarations::new(),
//         }
//     }

//     pub fn location(&self) -> &Location<'a> {
//         self.declarations.location()
//     }
// }

// impl<'a> Declarations<'a> {
//     pub fn new() -> Declarations<'a> {
//         Declarations {
//             declarations: Vec::new(),
//         }
//     }

//     pub fn location(&self) -> &Location<'a> {
//         unimplemented!()
//     }
// }

fn integer_node(loc: Location, val: String) -> IntegerLiteralNode {
    let i: i64 = integer_value(val.clone());
    if val.ends_with("UL") {
        IntegerLiteralNode::new(loc, Box::new(IntegerTypeRef::ulong_ref(loc)), i)
    } else if val.ends_with("L") {
        IntegerLiteralNode::new(loc, Box::new(IntegerTypeRef::long_ref(loc)), i)
    } else if val.ends_with("U") {
        IntegerLiteralNode::new(loc, Box::new(IntegerTypeRef::uint_ref(loc)), i)
    } else {
        IntegerLiteralNode::new(loc, Box::new(IntegerTypeRef::int_ref(loc)), i)
    }
}

fn integer_value(val: String) -> i64 {
    val.replace("U", "").replace("L", "").parse::<i64>().unwrap()
}

fn character_code(val: String) -> i64 {
    let mut s = string_value(val);
    assert!(s.len() == 1);
    s.pop().unwrap() as i64
}

fn string_value(val: String) -> String {
    unimplemented!()
}