use super::location::Location;
use super::type_::*;

const INDENT_STRING: &'static str = "    ";

pub trait Node {
    fn location(&self) -> Location;
    fn dump(&self, indent_level: usize) -> String;
}

macro_rules! impl_node_trait {
    ($t: ty, $self_: ident, $string: block) => (
        impl Node for $t {
            fn location(&self) -> Location {
                self.location
            }
            fn dump(&self, indent_level: usize) -> String {
                let indents: Vec<&'static str> = (0..indent_level)
                                                    .map(|_| INDENT_STRING)
                                                    .collect();
                let indent: String = indents.concat();
                let $self_ = self;
                format!("{}{}", indent, $string)
            }
        }
    )
}

macro_rules! define_node {
    ($node_name: ident; {
        $($member_name: ident: $member_type: ty,)*
    }; $self_: ident, $string: block) => (

        pub struct $node_name {
            location: Location,
            $($member_name: $member_type),*
        }

        impl $node_name {
            pub fn new(location: Location, $($member_name: $member_type),*) -> Self {
                $node_name {
                    location: location,
                    $($member_name: $member_name),*
                }
            }
        }

        impl_node_trait!($node_name, $self_, $string);
    )
}


define_node!(
    AST;
    {};
    self_, {
        format!("<<AST>> ({})\n", self_.location) +
        "variable: \n" +
        "function: \n"
    }
);

define_node!(
    IntegerLiteralNode;
    {
        type_: IntegerTypeRef,
        value: i64,
    };
    self_, {
        format!("<<IntegerLiteralNode>> ({})\n", self_.location) +
        &format!("typeNode: {:?}", self_.type_) +
        &format!("value: {}", self_.value)
    }
);

define_node!(
    BinaryOpNode;
    {
        left: Box<Node>,
        type_: BinaryOpType,
        right: Box<Node>,
    };
    self_, {
        format!("<<BinaryOpNode>> ({})\n", self_.location)
    }
);

define_node!(
    StringLiteralNode;
    {
        value: String,
    };
    self_, {
        format!("<<StringLiteralNode>> ({})\n", self_.location)
    }
);

define_node!(
    UnaryOpNode;
    {
        type_: UnaryOpType,
        node: Box<Node>,
    };
    self_, {
        format!("<<UnaryOpNode>> ({})\n", self_.location)
    }
);

define_node!(
    VariableNode;
    {
        name: String,
    };
    self_, {
        format!("<<VariableNode>> ({})\n", self_.location)
    }
);

define_node!(
    PrefixOpNode;
    {
        type_: PrefixOpType,
        node: Box<Node>,
    };
    self_, {
        format!("<<PrefixOpNode>> ({})\n", self_.location)
    }
);

define_node!(
    DereferenceNode;
    {
        node: Box<Node>,
    };
    self_, {
        format!("<<DereferenceNode>> ({})\n", self_.location)
    }
);

define_node!(
    AddressNode;
    {
        node: Box<Node>,
    };
    self_, {
        format!("<<AddressNode>> ({})\n", self_.location)
    }
);

define_node!(
    CastNode;
    {
        type_: Box<TypeRef>,
        node: Box<Node>,
    };
    self_, {
        format!("<<CastNode>> ({})\n", self_.location)
    }
);

define_node!(
    SizeofTypeNode;
    {
        type_: Box<TypeRef>,
        size: i64,
    };
    self_, {
        format!("<<SizeofTypeNode>> ({})\n", self_.location)
    }
);

define_node!(
    SizeofExprNode;
    {
        node: Box<Node>,
        size: i64,
    };
    self_, {
        format!("<<SizeofTypeNode>> ({})\n", self_.location)
    }
);

define_node!(
    SuffixOpNode;
    {
        type_: SuffixOpType,
        expr: Box<Node>,
    };
    self_, {
        format!("<<SuffixOpNode>> ({})\n", self_.location)
    }
);

define_node!(
    ArefNode;
    {
        expr: Box<Node>,
        idx: Box<Node>, // another expr
    };
    self_, {
        format!("<<ArefNode>> ({})\n", self_.location)
    }
);

define_node!(
    MemberNode;
    {
        expr: Box<Node>,
        memb: Box<Node>, // another expr
    };
    self_, {
        format!("<<MemberNode>> ({})\n", self_.location)
    }
);

define_node!(
    PtrMemberNode;
    {
        expr: Box<Node>,
        memb: Box<Node>, // another expr
    };
    self_, {
        format!("<<PtrMemberNode>> ({})\n", self_.location)
    }
);

define_node!(
    FuncallNode;
    {
        expr: Box<Node>,
        args: Vec<Box<Node>>, // another expr
    };
    self_, {
        format!("<<FuncallNode>> ({})\n", self_.location)
    }
);

define_node!(
    LogicalAndNode;
    {
        left: Box<Node>,
        right: Box<Node>,
    };
    self_, {
        format!("<<LogicalAndNode>> ({})\n", self_.location)
    }
);

define_node!(
    LogicalOrNode;
    {
        left: Box<Node>,
        right: Box<Node>,
    };
    self_, {
        format!("<<LogicalOrNode>> ({})\n", self_.location)
    }
);

define_node!(
    CondExprNode;
    {
        condition: Box<Node>,
        then_clause: Box<Node>,
        else_clause: Box<Node>,
    };
    self_, {
        format!("<<CondExprNode>> ({})\n", self_.location)
    }
);

define_node!(
    AssignNode;
    {
        lhs: Box<Node>,
        rhs: Box<Node>,
    };
    self_, {
        format!("<<AssignNode>> ({})\n", self_.location)
    }
);

define_node!(
    OpAssignNode;
    {
        lhs: Box<Node>,
        type_: OpAssignType,
        rhs: Box<Node>,
    };
    self_, {
        format!("<<OpAssignNode>> ({})\n", self_.location)
    }
);

define_node!(
    IfNode;
    {
        condition: Box<Node>,
        then_clause: Box<Node>,
        else_clause: Option<Box<Node>>,
    };
    self_, {
        format!("<<IfNode>> ({})\n", self_.location)
    }
);

define_node!(
    WhileNode;
    {
        condition: Box<Node>,
        body: Box<Node>,
    };
    self_, {
        format!("<<WhileNode>> ({})\n", self_.location)
    }
);

define_node!(
    DoWhileNode;
    {
        body: Box<Node>,
        condition: Box<Node>,
    };
    self_, {
        format!("<<DoWhileNode>> ({})\n", self_.location)
    }
);

define_node!(
    ForNode;
    {
        init_expr: Option<Box<Node>>,
        cond_expr: Option<Box<Node>>,
        step_expr: Option<Box<Node>>,
        body: Box<Node>,
    };
    self_, {
        format!("<<ForNode>> ({})\n", self_.location)
    }
);

define_node!(
    SwitchNode;
    {
        expr: Box<Node>,
        cases: Option<Box<Node>>,
    };
    self_, {
        format!("<<SwitchNode>> ({})\n", self_.location)
    }
);

define_node!(
    BreakNode;
    {};
    self_, {
        format!("<<BreakNode>> ({})\n", self_.location)
    }
);

define_node!(
    ContinueNode;
    {};
    self_, {
        format!("<<ContinueNode>> ({})\n", self_.location)
    }
);

define_node!(
    GotoNode;
    {
        label: String,
    };
    self_, {
        format!("<<GotoNode>> ({})\n", self_.location)
    }
);

define_node!(
    ReturnNode;
    {
        expr: Option<Box<Node>>,
    };
    self_, {
        format!("<<ReturnNode>> ({})\n", self_.location)
    }
);

define_node!(
    LabelNode;
    {
        label: String,
        stmt: Box<Node>,
    };
    self_, {
        format!("<<LabelNode>> ({})\n", self_.location)
    }
);

define_node!(
    CasesNode;
    {
        normal_cases: Vec<Box<Node>>,
        default_case: Option<Box<Node>>,
    };
    self_, {
        format!("<<CasesNode>> ({})\n", self_.location)
    }
);

define_node!(
    CaseNode;
    {
        expr: Box<Node>, // should be constant expr
        stmt: Vec<Box<Node>>,
    };
    self_, {
        format!("<<CaseNode>> ({})\n", self_.location)
    }
);

define_node!(
    DefaultCaseNode;
    {
        stmt: Vec<Box<Node>>,
    };
    self_, {
        format!("<<DefaultCaseNode>> ({})\n", self_.location)
    }
);

trait ExprNode: Node {}

trait AssignNodeTrait: ExprNode {}

trait BinaryOpTrait: ExprNode {}

trait LHSNode: ExprNode {}

trait LiteralNode: ExprNode {}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpType {
    Plus,
    Hyphen,
    ExclamationMark,
    Tilde,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOpType {
    Multiplication,
    Division,
    Modulo,
    Addition,
    Subtraction,
    LeftShift,
    RightShift,
    BitAnd,
    BitOr,
    BitExclusiveOr,
    GreaterThan,
    LessThan,
    DoubleEquals,
    NotEqualTo,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

#[derive(Debug, Copy, Clone)]
pub enum OpAssignType {
    AddAssignment,
    SubtractAssignment,
    MultiplyAssignment,
    DivideAssignment,
    ModuloAssignment,
    AndAssignment,
    ExclusiveOrAssignment,
    OrAssignment,
    LeftShiftAssignment,
    RightShiftAssignment,
}

trait UnaryArithmeticOpNode {}

#[derive(Debug, Copy, Clone)]
pub enum PrefixOpType {
    Increment,
    Decrement,
}

#[derive(Debug, Copy, Clone)]
pub enum SuffixOpType {
    Increment,
    Decrement,
}

pub struct Slot {}

trait StmtNode: Node {}

pub struct BlockNode {}

pub struct ExprStmtNode {}

trait TypeDefinition: Node {}

trait CompositeTypeDefinition: TypeDefinition {}

pub struct StructNode {}

pub struct UnionNode {}

trait TypedefNode: TypeDefinition {}


pub mod helper {
    use super::*;

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
        val.replace("U", "")
            .replace("L", "")
            .parse::<i64>()
            .unwrap()
    }

    pub fn character_code(val: String) -> i64 {
        assert!(val.len() == 1);
        val.chars().next().unwrap() as i64
    }
}

