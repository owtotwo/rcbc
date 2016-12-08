use std::path::PathBuf;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct AST {
    source: Location,
    declarations: Declarations,
}

#[derive(Debug, Clone)]
pub struct Location {
    file: PathBuf,
    begin_line: usize,
    begin_column: usize,
    end_line: usize,
    end_column: usize,
}

#[derive(Debug, Clone)]
pub struct Declarations {
    defvars: BTreeSet<DefinedVariable>,
    vardecls: BTreeSet<UndefinedVariable>,
    defuns: BTreeSet<DefinedFunction>,
    funcdecls: BTreeSet<UndefinedFunction>,
    constants: BTreeSet<Constant>,
    defstructs: BTreeSet<StructNode>,
    defunions: BTreeSet<UnionNode>,
    typedefs: BTreeSet<TypedefNode>,
}

#[derive(Debug, Clone)]
pub struct UndefinedVariable {
    type_node: TypeNode,
    name: String,
    is_private: bool,
}

#[derive(Debug, Clone)]
pub struct DefinedVariable {
    type_node: TypeNode,
    name: String,
    is_private: bool,
    initializer: ExprNode,
}

#[derive(Debug, Clone)]
pub struct DefinedFunction {
    type_node: TypeNode,
    name: String,
    is_private: bool,
    params: Params,
    body: BlockNode,
}

#[derive(Debug, Clone)]
pub struct UndefinedFunction {
    type_node: TypeNode,
    name: String,
    is_private: bool,
    params: Params,
}

#[derive(Debug, Clone)]
pub struct Constant {
    type_node: TypeNode,
    name: String,
    value: ExprNode,
}

#[derive(Debug, Clone)]
pub struct StructNode {
    location: Location,
    type_ref: TypeRef,
    name: String,
    membs: Vec<Slot>,
}

#[derive(Debug, Clone)]
pub struct UnionNode {
    location: Location,
    type_ref: TypeRef,
    name: String,
    membs: Vec<Slot>,
}

#[derive(Debug,  Clone)]
pub struct TypedefNode {
    location: Location,
    name: String,
    real: TypeNode,
}


#[derive(Debug, Clone)]
pub struct TypeNode {
    type_ref: TypeRef,
    self_type: Type,
}

#[derive(Debug,  Clone)]
pub struct TypeRef {
    location: Location,
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
pub struct Params {
    location: Location,
    param_descs: Vec<Parameter>,
}

#[derive(Debug,  Clone)]
pub struct BlockNode {
    location: Location,
    variables: Vec<DefinedVariable>,
    stmts: Vec<StmtNode>,
}


#[derive(Debug,  Clone)]
pub struct Slot {
    type_node: TypeNode,
    name: String,
    offset: usize,
}

#[derive(Debug,  Clone)]
pub struct Parameter {
    type_node: TypeNode,
    name: String,
}

#[derive(Debug,  Clone)]
pub struct StmtNode {
    location: Location,
}