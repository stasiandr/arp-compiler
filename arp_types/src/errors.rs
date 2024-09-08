use std::ops::Range;



pub struct DiagnosticError {
    range: Range<usize>,
    message: Box<str>,
}

impl DiagnosticError {
    pub fn new<M : Into<Box<str>>>(range: Range<usize>, message: M) -> Self {
        Self { range, message: message.into(), }
    }
    
    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
    
    pub fn message(&self) -> &str {
        &self.message
    }
}