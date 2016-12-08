use super::location::Location;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct AST<'a> {
    source: Location<'a>,
    declarations: Declarations<'a>,
}

#[derive(Debug, Clone)]
pub struct Declarations<'a> {
    defvars: BTreeSet<DefinedVariable<'a>>,
    vardecls: BTreeSet<UndefinedVariable<'a>>,
    defuns: BTreeSet<DefinedFunction<'a>>,
    funcdecls: BTreeSet<UndefinedFunction<'a>>,
    constants: BTreeSet<Constant<'a>>,
    defstructs: BTreeSet<StructNode<'a>>,
    defunions: BTreeSet<UnionNode<'a>>,
    typedefs: BTreeSet<TypedefNode<'a>>,
}

#[derive(Debug, Clone)]
pub struct UndefinedVariable<'a> {
    type_node: TypeNode<'a>,
    name: String,
    is_private: bool,
}

#[derive(Debug, Clone)]
pub struct DefinedVariable<'a> {
    type_node: TypeNode<'a>,
    name: String,
    is_private: bool,
    initializer: ExprNode,
}

#[derive(Debug, Clone)]
pub struct DefinedFunction<'a> {
    type_node: TypeNode<'a>,
    name: String,
    is_private: bool,
    params: Params<'a>,
    body: BlockNode<'a>,
}

#[derive(Debug, Clone)]
pub struct UndefinedFunction<'a> {
    type_node: TypeNode<'a>,
    name: String,
    is_private: bool,
    params: Params<'a>,
}

#[derive(Debug, Clone)]
pub struct Constant<'a> {
    type_node: TypeNode<'a>,
    name: String,
    value: ExprNode,
}

#[derive(Debug, Clone)]
pub struct StructNode<'a> {
    location: Location<'a>,
    type_ref: TypeRef<'a>,
    name: String,
    membs: Vec<Slot<'a>>,
}

#[derive(Debug, Clone)]
pub struct UnionNode<'a> {
    location: Location<'a>,
    type_ref: TypeRef<'a>,
    name: String,
    membs: Vec<Slot<'a>>,
}

#[derive(Debug,  Clone)]
pub struct TypedefNode<'a> {
    location: Location<'a>,
    name: String,
    real: TypeNode<'a>,
}


#[derive(Debug, Clone)]
pub struct TypeNode<'a> {
    type_ref: TypeRef<'a>,
    self_type: Type,
}

#[derive(Debug,  Clone)]
pub struct TypeRef<'a> {
    location: Location<'a>,
}

#[derive(Debug,  Clone)]
pub struct Type {
    // ...
}


#[derive(Debug,  Clone)]
pub struct ExprNode {
    // ...
}

#[derive(Debug,  Clone)]
pub struct Params<'a> {
    location: Location<'a>,
    param_descs: Vec<Parameter<'a>>,
}

#[derive(Debug,  Clone)]
pub struct BlockNode<'a> {
    location: Location<'a>,
    variables: Vec<DefinedVariable<'a>>,
    stmts: Vec<StmtNode<'a>>,
}


#[derive(Debug,  Clone)]
pub struct Slot<'a> {
    type_node: TypeNode<'a>,
    name: String,
    offset: usize,
}

#[derive(Debug,  Clone)]
pub struct Parameter<'a> {
    type_node: TypeNode<'a>,
    name: String,
}

#[derive(Debug,  Clone)]
pub struct StmtNode<'a> {
    location: Location<'a>,
}