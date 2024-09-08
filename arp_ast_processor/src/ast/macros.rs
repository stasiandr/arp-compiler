#[macro_export]
macro_rules! impl_from_node_kind {
    ($union:ty, $p:path, $child:ty) => {
        impl From<$child> for $union {
            fn from(value: $child) -> Self {
                $p(value)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from {
    ($union:ty, $p:path, $child:ty) => {
        impl TryFrom<$union> for $child {
            type Error = super::errors::AstError;
            
            fn try_from(value: $union) -> Result<Self, Self::Error> {
                match value {
                    $p(lit) => Ok(lit),
                    ref value => Err(Self::Error::WrongNodeType { 
                        expected: std::any::type_name::<$child>().to_string(), 
                        actual: format!("{:?}", value).to_string() 
                    })
                }
            }    
        }
    };
}

#[macro_export]
macro_rules! impl_as_ref {
    ($union:ty, $p:path, $child:ty) => {
        impl AsRef<$child> for $union {
            fn as_ref(&self) -> &$child {
                match self {
                    $p(lit) => lit,
                    ref v => panic!("{:?}", v),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_mut {
    ($union:ty, $p:path, $child:ty) => {
        impl AsMut<$child> for $union {
            fn as_mut(&mut self) -> &mut $child {
                match self {
                    $p(lit) => lit,
                    ref v => panic!("{:?}", v),
                }
            }
        }
    };
}

// #[macro_export]
// macro_rules! derive_implementations {
//     ($union:ty, $p:path, $child:ty) => {
//         impl_from_node_kind!($union, $p, $child);
//         impl_try_from!($union, $p, $child);
//         impl_as_ref!($union, $p, $child);
//         impl_as_mut!($union, $p, $child);
//     };
// }


#[macro_export]
macro_rules! derive_implementations {
    ($union:ty, $p:path, $child:ty) => {

        impl $crate::ast::traits::AstNodeKind<$union> for $child { }

        impl From<$child> for $union {
            fn from(value: $child) -> Self {
                $p(value)
            }
        }


        impl TryFrom<$union> for $child {
            type Error = $crate::ast::errors::AstError;
            
            fn try_from(value: $union) -> Result<Self, Self::Error> {
                match value {
                    $p(lit) => Ok(lit),
                    ref value => Err(Self::Error::WrongNodeType { 
                        expected: std::any::type_name::<$child>().to_string(), 
                        actual: format!("{:?}", value).to_string() 
                    })
                }
            }    
        }


        impl AsRef<$child> for $union {
            fn as_ref(&self) -> &$child {
                match self {
                    $p(lit) => lit,
                    ref v => panic!("{:?}", v),
                }
            }
        }


        impl AsMut<$child> for $union {
            fn as_mut(&mut self) -> &mut $child {
                match self {
                    $p(lit) => lit,
                    ref v => panic!("{:?}", v),
                }
            }
        }
    };
}