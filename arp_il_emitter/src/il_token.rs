use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub enum ILToken {
    OpCode(OpCode),

    StartMethod(Method),
    EndMethod(String),

    StartStructure(HashSet<StructureFlags>, String),
    EndStructure(String),

    Field(String, ResolvedType),

    #[default]
    Empty,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum OpCode {
    LabeledOpCode(String, Box<OpCode>),

    LoadInt(i64),
    LoadFloat(f64),
    LoadString(String),
    LoadBool(bool),
    LoadLocalVariable(usize),
    StoreLocalVariable(usize),
    LoadArgument(usize),
    StoreArgument(usize),
    Call { 
        is_instance: bool,
        return_type: ResolvedType,
        external: Option<String>,
        ty: String,
        method_name: String,
        args: Vec<ResolvedType>
    },
    NewObject(ResolvedType, Vec<ResolvedType>),

    SetField(ResolvedType, String, String),
    GetField(ResolvedType, String, String),

    BranchIfFalse(String),
    BranchIfTrue(String),
    BranchTo(String),

    Add,
    Multiply,
    Subtract,
    Divide,

    Equal,
    LessThen,
    GreaterThen,

    #[default]
    NoOperation,
    Or,
    And,
    Return,
    
    
    
}

impl From<OpCode> for ILToken {
    fn from(value: OpCode) -> Self {
        Self::OpCode(value)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Method {
    pub flags: HashSet<FunctionFlags>,
    pub params: Vec<(String, ResolvedType)>,
    pub registers: Vec<ResolvedType>,
    pub return_ty: ResolvedType,
    pub name: String,
}

impl From<Method> for ILToken {
    fn from(value: Method) -> Self {
        Self::StartMethod(value)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResolvedType(pub String);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum FunctionFlags {
    Cil,
    Managed,
    IsStatic(bool),
    EntryPoint,

    #[default]
    Nothing,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum StructureFlags {
    Auto,


    #[default]
    Nothing,
}

// let mut flags = vec![
//     FunctionFlags::CIL,
//     FunctionFlags::MANAGED,
//     match func.kind {
//         FunctionKind::Method { .. } => FunctionFlags::INSTANCE,
//         FunctionKind::Static => FunctionFlags::STATIC,
//     },
// ];

// if is_entrypoint {
//     flags.push(FunctionFlags::ENTRY_POINT);
// }
