use super::location::Location;

trait Node {
    fn location(&self) -> &Location;
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