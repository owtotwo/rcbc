use super::location::Location;

pub trait Type {
    // ...
}

pub struct ArrayType {
    // ...
}

pub struct FunctionType {
    // ...
}

pub struct IntegerType {
    // ...
}

pub struct NamedType {
    // ...
}

pub struct CompositeType {
    // ...
}

pub struct StructType {
    // ...
}

pub struct UnionType {
    // ...
}

pub struct UserType {
    // ...
}

pub struct PointerType {
    size: usize,
    base_type: Box<Type>
}

pub struct VoidType {
    // ...
}


pub trait TypeRef {
    // ...
}

pub struct ArrayTypeRef {
    // ...
}

pub struct FunctionTypeRef {
    // ...
}

pub struct IntegerTypeRef {
    location: Location,
    name: String,
}

pub struct NamedTypeRef {
    // ...
}

pub struct CompositeTypeRef {
    // ...
}

pub struct StructTypeRef {
    // ...
}

pub struct UnionTypeRef {
    // ...
}

pub struct UserTypeRef {
    // ...
}

pub struct PointerTypeRef {
    base_type: Box<TypeRef>
}

pub struct VoidTypeRef {
    // ...
}


macro_rules! impl_type_ref {
    ($T: ident, $Str: expr) => (
        impl IntegerTypeRef {
            pub fn $T(loc: Location) -> IntegerTypeRef {
                IntegerTypeRef::new($Str.to_string(), loc)
            }
        }
    )
}

impl_type_ref!(char_ref, "char");
impl_type_ref!(short_ref, "short");
impl_type_ref!(int_ref, "int");
impl_type_ref!(long_ref, "long");
impl_type_ref!(uchar_ref, "unsigned char");
impl_type_ref!(ushort_ref, "unsigned short");
impl_type_ref!(uint_ref, "unsigned int");
impl_type_ref!(ulong_ref, "unsigned long");


impl IntegerTypeRef {
    fn new(s: String, loc: Location) -> IntegerTypeRef {
        IntegerTypeRef {
            location: loc,
            name: s
        }
    }
}

impl TypeRef for IntegerTypeRef {
    
}