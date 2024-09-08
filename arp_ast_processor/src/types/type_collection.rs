use crate::type_resolver::managed_dll_info::SharpTypeInfo;

use super::{ast_node_value::Id, function::Function, simple::Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeCollection {
    collection: Vec<Type>,
}

impl Default for TypeCollection {
    fn default() -> Self {
        Self { collection: TypeInfo::standard_types() }
    }
}


impl TypeCollection {
    #[inline]
    pub fn get(&self, id: &StrongTypeId) -> &TypeInfo {
        match &self.collection[id.index] {
            Type::Resolved(ty) => ty,
            Type::PlaceHolder(_) => unreachable!("typeId only issued for already resolved types"),
        }
    }

    #[inline]
    pub fn get_name(&self, id: &TypeId) -> Option<&str> {
        if let Some(index) = id.get_index() {
            Some(self.collection[index].get_name())
        } else {
            None
        }
    }

    pub fn try_get_strong(&self, id: &TypeId) -> Option<&TypeInfo> {
        match id {
            TypeId::Strong(s) => self.collection.get(s.index).and_then(|t| match t {
                Type::Resolved(full) => Some(full),
                Type::PlaceHolder(_) => None,
            }),
            TypeId::Weak(w) => self.collection.get(w.index).and_then(|t| match t {
                Type::Resolved(full) => Some(full),
                Type::PlaceHolder(_) => None,
            }),
            TypeId::None => None,
        }
    }

    pub fn resolve_name<S : AsRef<str> + ?Sized>(&self, name: &S) -> TypeId {
        self.collection
            .iter()
            .enumerate()
            .find(|(_, ty)| match ty {
                Type::Resolved(ty) => {
                    if let Some(short_name) = &ty.short_name {
                        short_name == name.as_ref() || ty.full_name.as_ref() == name.as_ref()
                    } else {
                        ty.full_name.as_ref() == name.as_ref()
                    }
                },
                Type::PlaceHolder(n) => n == name.as_ref(),
            })
            .map(|(index, ty)| {
                match ty {
                    Type::Resolved(_) => StrongTypeId::new(index).into(),
                    Type::PlaceHolder(_) => WeakTypeId::new(index).into(),
                }
            }).unwrap_or_default()
    }

    pub fn copy_from(&mut self, type_info: &TypeInfo, path: &str) {
        let info = TypeInfo {
            full_name: type_info.full_name.clone(),
            short_name: None,
            source: TypeSourceKind::ExternalArp(path.to_owned()),
            fields: type_info.fields.clone(),
            methods: type_info.methods.clone(),
        };

        match self.resolve_name(&info.full_name) {
            TypeId::Strong(_) => { },
            TypeId::Weak(n) => {
                self.collection[n.index] = Type::Resolved(info);
            },
            TypeId::None => {
                self.collection.push(Type::Resolved(info));
            },
        }
    }

    pub fn insert_external<P : AsRef<str>>(&mut self, path: P, external: &SharpTypeInfo) {
        
        let mut info = TypeInfo {
            full_name: external.full_name.clone().into(),
            short_name: external.short_name.clone(),
            source: TypeSourceKind::ManagedDll(path.as_ref().into()),
            fields: vec![],
            methods: vec![],
        };

        for fld in external.fields.iter() {
            info.fields.push((fld.ident.clone().into(), TypeId::None))
        }

        for mtd in external.methods.iter() {
            info.methods.push(MethodInfo {
                name: mtd.ident.clone().into(),
                args: mtd.args.iter().map(|arg| {
                    (arg.ident.clone().into(), self.resolve_name(&arg.ty_full_name))
                }).collect(),
                return_type: self.resolve_name(&mtd.return_ty_full_name),
                definition: None,
            })
        }

        // let info = dbg!(info);

        match self.resolve_name(&info.full_name) {
            TypeId::Strong(_) => { },
            TypeId::Weak(n) => {
                self.collection[n.index] = Type::Resolved(info);
            },
            TypeId::None => {
                self.collection.push(Type::Resolved(info));
            },
        }
    }
    
    pub(crate) fn get_or_allocate<S : AsRef<str>>(&mut self, name: &S) -> TypeId {
        let name = name.as_ref();
        
        let ty = self.resolve_name(name);

        if ty.is_none() {
            self.collection.push(Type::PlaceHolder(name.into()));
            WeakTypeId::new(self.collection.len() - 1).into()
        } else {
            ty
        }
    }
    
    pub(crate) fn try_allocate(&mut self, weak_id: WeakTypeId, fields: Vec<(Identifier, TypeId)>) -> TypeId {
        let info = TypeInfo {
            full_name: self.collection[weak_id.index].get_name().into(),
            short_name: None,
            source: TypeSourceKind::LocalArp,
            fields,
            methods: vec![],
        };

        match self.resolve_name(&info.full_name) {
            TypeId::Strong(_) => { TypeId::None },
            TypeId::Weak(n) => {
                self.collection[n.index] = Type::Resolved(info);
                StrongTypeId::new(n.index).into()
            },
            TypeId::None => {
                self.collection.push(Type::Resolved(info));
                StrongTypeId::new(self.collection.len() - 1).into()
            },
        }
    }
    
    pub(crate) fn resolve_recursive(&mut self, self_type: TypeId) {
        if let Some(id) = self_type.try_into_strong() {
            match &mut self.collection[id.index] {
                Type::Resolved(res) => {
                    for index in 0..res.fields.len() {
                        if res.fields[index].1.get_index() == self_type.get_index() {
                            res.fields[index].1 = TypeId::Strong(id.clone());
                        }
                    }
                },
                Type::PlaceHolder(_) => {},
            }
        }
    }
    
    pub(crate) fn extend_type_methods(&mut self, impl_type: TypeId, functions: Vec<(Id<Function>, Function)>) {
        if let Some(Type::Resolved(ty)) = self.collection.get_mut(impl_type.get_index().unwrap()) {
            for (index, func) in functions {
                ty.methods.push(MethodInfo {
                    name: func.name,
                    args: func.parameters,
                    return_type: func.return_type,
                    definition: Some(index),
                });
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Resolved(TypeInfo),
    PlaceHolder(String),
}

impl Type {
    pub fn get_name(&self) -> &str {
        match self {
            Type::Resolved(ty) => &ty.full_name,
            Type::PlaceHolder(b) => b,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum TypeId {
    Strong(StrongTypeId),
    Weak(WeakTypeId),

    #[default]
    None
}
impl TypeId {
    fn get_index(&self) -> Option<usize> {
        match self {
            TypeId::Strong(s) => Some(s.index),
            TypeId::Weak(w) => Some(w.index),
            TypeId::None => None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, TypeId::None)
    }

    pub fn try_into_none(&self) -> Option<()> {
        match self {
            TypeId::None => Some(()),
            _ => None
        }
    }

    pub fn is_strong(&self) -> bool {
        matches!(self, TypeId::Strong(_))
    }

    pub fn try_into_strong(&self) -> Option<&StrongTypeId>  {
        match self {
            TypeId::Strong(s) => Some(s),
            _ => None
        }
    }


    pub fn is_weak(&self) -> bool {
        matches!(self, TypeId::Weak(_))
    }

    pub fn try_into_weak(&self) -> Option<&WeakTypeId> {
        match self {
            TypeId::Weak(s) => Some(s),
            _ => None
        }
    }
}

impl From<StrongTypeId> for TypeId  {
    fn from(value: StrongTypeId) -> Self {
        TypeId::Strong(value)
    }
}

impl From<WeakTypeId> for TypeId  {
    fn from(value: WeakTypeId) -> Self {
        TypeId::Weak(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StrongTypeId {
    pub(self) index: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct WeakTypeId {
    pub(self) index: usize
}


impl WeakTypeId {
    #[inline]
    pub(crate) fn new(index: usize) -> Self {
        Self { 
            index, 
        }
    }
}

impl StrongTypeId {
    #[inline]
    pub(crate) fn new(index: usize) -> Self {
        Self { 
            index, 
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeInfo {
    pub full_name: Box<str>,
    pub short_name: Option<String>,
    pub source: TypeSourceKind,

    pub fields: Vec<(Identifier, TypeId)>,
    pub methods: Vec<MethodInfo>
}


impl TypeInfo {
    pub fn find_method(&self, name: &Identifier, arg_types: Vec<TypeId>) -> Option<&MethodInfo> {

        // dbg!(&self.methods);

        for (index, mi) in self.methods.iter().enumerate() {
            if &mi.name == name && mi.args.len() == arg_types.len() && mi.args.iter()
                    .map(|(_, ty)| ty)
                    .zip(arg_types.iter())
                    .all(|(lhs, rhs)| lhs == rhs) {
                return self.methods.get(index);
            }
        }

        None
    }
    
}


#[derive(Debug, PartialEq, Clone)]
pub struct MethodInfo {
    pub name: Identifier,
    pub args: Vec<(Identifier, TypeId)>,
    pub return_type: TypeId,

    pub definition: Option<Id<Function>>
}


#[derive(Debug, PartialEq, Clone)]
pub enum TypeSourceKind {
    LocalArp,
    ExternalArp(String),
    Standard,
    ManagedDll(String)
}


impl TypeCollection {
    #[inline]
    pub fn get_void(&self) -> TypeId {
        self.resolve_name("void")
    }

    #[inline]
    pub fn get_int(&self) -> TypeId {
        self.resolve_name("int32")
    }

    #[inline]
    pub fn get_float(&self) -> TypeId {
        self.resolve_name("float32")
    }

    #[inline]
    pub fn get_bool(&self) -> TypeId {
        self.resolve_name("bool")
    }

    #[inline]
    pub fn get_string(&self) -> TypeId {
        self.resolve_name("string")
    }
}

impl TypeInfo {
    #[inline]
    pub fn standard_types() -> Vec<Type> {
        vec![
            Type::Resolved(Self::void()),
            Type::Resolved(Self::int()),
            Type::Resolved(Self::float()),
            Type::Resolved(Self::bool()),
            Type::Resolved(Self::string()),
        ]
    }

   #[inline]
   pub fn void() -> Self {
        Self {
            full_name: "System.Void".into(),
            short_name: Some("void".into()),
            source: TypeSourceKind::Standard,
            fields: vec![],
            methods: vec![],
        }
    }

    #[inline]
    pub fn int() -> Self {
        Self {
            full_name: "System.Int32".into(),
            short_name: Some("int32".into()),
            source: TypeSourceKind::Standard,
            fields: vec![],
            methods: vec![],
        }
    }

    #[inline]
    pub fn float() -> Self {
        Self {
            full_name: "System.Float32".into(),
            short_name: Some("float32".into()),
            source: TypeSourceKind::Standard,
            fields: vec![],
            methods: vec![],
        }
    }

    #[inline]
    pub fn bool() -> Self {
        Self {
            full_name: "System.Boolean".into(),
            short_name: Some("bool".into()),
            source: TypeSourceKind::Standard,
            fields: vec![],
            methods: vec![],
        }
    }

    #[inline]
    pub fn string() -> Self {
        Self {
            full_name: "System.String".into(),
            short_name: Some("string".into()),
            source: TypeSourceKind::Standard,
            fields: vec![],
            methods: vec![],
        }
    }
}
