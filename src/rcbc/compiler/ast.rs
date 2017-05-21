use super::location::Location;
use super::type_::*;

const INDENT_STRING: &'static str = "    ";

pub trait Node {
    fn location(&self) -> Location;
    fn dump(&self, indent_level: usize) -> String;
}

macro_rules! impl_node_trait {
    ($t: ty, $string: block) => (
        impl Node for $t {
            fn location(&self) -> Location {
                self.location
            }
            fn dump(&self, indent_level: usize) -> String {
                let indents: Vec<&'static str> = (0..indent_level)
                                                    .map(|_| INDENT_STRING)
                                                    .collect();
                let indent: String = indents.concat();
                format!("{}{}", indent, $string)
            }
        }
    )
}

macro_rules! define_node {
    ($node_name: ident; {
        $($member_name: ident: $member_type: ty),*
    }; $string: block) => (
        pub struct $node_name {
            location: Location,
            $($member_name: $member_type),*
        }

        impl $node_name {
            fn new(location: Location, $($member_name: $member_type),*) -> Self {
                $node_name {
                    location: location,
                    $($member_name: $member_name),*
                }
            }
        }

        impl_node_trait!($node_name, $string);
    )
}


define_node!(NewNode; { name: String }; { "<NewNode>..." });

trait ExprNode: Node {}

trait AssignNodeTrait: ExprNode {}

pub struct AssignNode {}

pub struct OpAssignNode {}

pub struct AddressNode {
    location: Location,
    node: Box<Node>,
}

trait BinaryOpTrait: ExprNode {}

pub struct BinaryOpNode {
    location: Location,
    operator: String,
    left: Box<ExprNode>,
    right: Box<ExprNode>,
    // type_: Box<Type>,
}

impl BinaryOpNode {
    fn new(left: Box<ExprNode>, op: String, right: Box<ExprNode>) -> BinaryOpNode {
        BinaryOpNode {
            location: Location {
                begin: left.location().begin,
                end: right.location().end,
            },
            left: left,
            operator: op,
            right: right,
        }
    }
}

pub struct LogicalAndNode {}

pub struct LogicalOrNode {}

pub struct CastNode {
    location: Location,
    type_: Box<TypeRef>,
    node: Box<Node>,
}

pub struct CondExprNode {}

pub struct FuncallNode {}

trait LHSNode: ExprNode {}

pub struct ArefNode {}

pub struct DereferenceNode {
    location: Location,
    node: Box<Node>,
}

pub struct MemberNode {}

pub struct PtrMemberNode {}

pub struct VariableNode {
    location: Location,
    name: String,
}

trait LiteralNode: ExprNode {}

pub struct IntegerLiteralNode {
    location: Location,
    type_: IntegerTypeRef,
    value: i64,
}

pub struct StringLiteralNode {
    location: Location,
    value: String,
}

pub struct SizeofExprNode {}

pub struct SizeofTypeNode {}

pub enum UnaryOpType {
    Plus,
    Hyphen,
    ExclamationMark,
    Tilde,
}

pub struct UnaryOpNode {
    location: Location,
    type_: UnaryOpType,
    node: Box<Node>,
}

trait UnaryArithmeticOpNode {}

pub enum PrefixOpType {
    Increment,
    Decrement,
}

pub struct PrefixOpNode {
    location: Location,
    type_: PrefixOpType,
    node: Box<Node>,
}

pub struct SuffixOpNode {}

pub struct Slot {}

trait StmtNode: Node {}

pub struct BlockNode {}

pub struct BreakNode {}

pub struct CaseNode {
    location: Location,
    type_: Box<TypeRef>,
    node: Box<Node>,
}

pub struct ContinueNode {}

pub struct DoWhileNode {}

pub struct ExprStmtNode {}

pub struct ForNode {}

pub struct GotoNode {}

pub struct IfNode {}

pub struct LabelNode {}

pub struct ReturnNode {}

pub struct SwitchNode {}

pub struct WhileNode {}

trait TypeDefinition: Node {}

trait CompositeTypeDefinition: TypeDefinition {}

pub struct StructNode {}

pub struct UnionNode {}

trait TypedefNode: TypeDefinition {}

pub enum TypeNode {
    Type(Box<Type>),
    TypeRef(Box<TypeRef>),
}


#[derive(Debug, Clone)]
pub struct AST {
    location: Location,
}

impl AST {
    pub fn new(location: Location) -> AST {
        AST {
            location: location,
        }
    }
}

impl Node for AST {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for IntegerLiteralNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for StringLiteralNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for VariableNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for PrefixOpNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for UnaryOpNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for DereferenceNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for AddressNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl Node for CastNode {
    fn location(&self) -> Location {
        self.location
    }

    fn dump(&self, indent_level: usize) -> String {
        unimplemented!()
    }
}

impl TypeNode {
    pub fn new(typeref: Box<TypeRef>) -> TypeNode {
        TypeNode::TypeRef(typeref)
    }
}

impl IntegerLiteralNode {
    pub fn new(location: Location, type_: IntegerTypeRef, value: i64) -> Self {
        IntegerLiteralNode {
            location: location,
            type_: type_,
            value: value,
        }
    }
}

impl StringLiteralNode {
    pub fn new(location: Location, value: String) -> Self {
        StringLiteralNode {
            location: location,
            value: value
        }
    }
}

impl VariableNode {
    pub fn new(location: Location, name: String) -> Self {
        VariableNode {
            location: location,
            name: name
        }
    }
}

impl PrefixOpNode {
    pub fn new(location: Location, type_: PrefixOpType, node: Box<Node>) -> Self {
        PrefixOpNode {
            location: location,
            type_: type_,
            node: node,
        }
    }
}

impl UnaryOpNode {
    pub fn new(location: Location, type_: UnaryOpType, node: Box<Node>) -> Self {
        UnaryOpNode {
            location: location,
            type_: type_,
            node: node,
        }
    }
}

impl DereferenceNode {
    pub fn new(location: Location, node: Box<Node>) -> Self {
        DereferenceNode {
            location: location,
            node: node,
        }
    }
}

impl AddressNode {
    pub fn new(location: Location, node: Box<Node>) -> Self {
        AddressNode {
            location: location,
            node: node,
        }
    }
}

impl CastNode {
    pub fn new(location: Location, type_: Box<TypeRef>, node: Box<Node>) -> Self {
        CastNode {
            location: location,
            type_: type_,
            node: node,
        }
    }
}

pub fn integer_node(location: Location, value: String) -> IntegerLiteralNode {
    let i: i64 = integer_value(value.clone());
    if value.ends_with("UL") {
        IntegerLiteralNode::new(location, IntegerTypeRef::UnsignedLong, i)
    } else if value.ends_with("L") {
        IntegerLiteralNode::new(location, IntegerTypeRef::Long, i)
    } else if value.ends_with("U") {
        IntegerLiteralNode::new(location, IntegerTypeRef::UnsignedInt, i)
    } else {
        IntegerLiteralNode::new(location, IntegerTypeRef::Int, i)
    }
}

fn integer_value(val: String) -> i64 {
    val.replace("U", "").replace("L", "").parse::<i64>().unwrap()
}

pub fn character_code(val: String) -> i64 {
    let mut s = string_value(val);
    assert!(s.len() == 1);
    s.pop().unwrap() as i64
}

pub fn string_value(val: String) -> String {
    unimplemented!()
}