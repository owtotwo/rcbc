use super::location::Location;

// ---------- Type ----------

pub trait Type {}

pub struct ArrayType {}

pub struct FunctionType {}

pub struct IntegerType {}

pub struct NamedType {}

pub struct CompositeType {}

pub struct StructType {}

pub struct UnionType {}

pub struct UserType {}

pub struct PointerType {
    size: usize,
    base_type: Box<Type>,
}

pub struct VoidType {}


// ---------- TypeRef ----------

pub trait TypeRef {}

pub struct ArrayTypeRef {}

pub struct FunctionTypeRef {}

#[derive(Debug, Copy, Clone)]
pub enum IntegerTypeRef {
    Char,
    Short,
    Int,
    Long,
    UnsignedChar,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
}

pub struct NamedTypeRef {}

pub struct CompositeTypeRef {}

pub struct StructTypeRef {}

pub struct UnionTypeRef {}

pub struct UserTypeRef {}

pub struct PointerTypeRef {
    base_type: Box<TypeRef>,
}

pub struct VoidTypeRef {}

impl TypeRef for IntegerTypeRef {}

