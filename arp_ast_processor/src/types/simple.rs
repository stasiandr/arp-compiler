
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub Box<str>);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.into())
    }
}
impl From<Box<str>> for Identifier {
    fn from(value: Box<str>) -> Self {
        Identifier(value)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value.into())
    }
}


impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0.to_string()
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}