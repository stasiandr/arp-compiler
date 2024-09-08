use arp_lexer::tokens::Float;
use arp_types::Spanned;



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChumskyNode {

    File(Vec<Spanned<Self>>),

    // Declarations
    ImportDecl(bool, Vec<Spanned<Self>>, Vec<Spanned<Self>>),
    ImplementationDecl(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    Structure(Box<Spanned<Self>>, Vec<Spanned<Self>>, Vec<Spanned<Self>>),
    FuncDecl(Box<Spanned<Self>>, Vec<Spanned<Self>>, Option<Box<Spanned<Self>>>, Box<Spanned<Self>>),
    VariableDecl(bool, Box<Spanned<Self>>, Option<Box<Spanned<Self>>>, Box<Spanned<Self>>),
    StatementDecl(Box<Spanned<Self>>),


    // Statements
    ExpressionStmt(Box<Spanned<Self>>),
    AssignmentStmt(Box<Spanned<Self>>, Box<Spanned<Self>>),
    IfStmt(Box<Spanned<Self>>, Box<Spanned<Self>>, Vec<(Spanned<Self>, Spanned<Self>)>, Option<Box<Spanned<Self>>>),
    WhileStmt(Box<Spanned<Self>>, Box<Spanned<Self>>),
    ForStmt(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    BlockStmt(Vec<Spanned<Self>>, Option<Box<Spanned<Self>>>),
    ReturnStmt(Box<Spanned<Self>>),

    
    // Expressions 
    GetExpr(Box<Spanned<Self>>, Box<Spanned<Self>>),
    UnaryExpr(UnaryOp, Box<Spanned<Self>>),
    BinaryExpr(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    CallExpr(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    ConstructExpr(Box<Spanned<Self>>, Vec<(Option<Spanned<Self>>, Spanned<Self>)>),
    ArrayExpr(Vec<Spanned<Self>>),


    // Atoms
    This, Base,
    Break,
    LiteralInteger(i64),
    LiteralFloat(Float),
    LiteralString(Box<str>),
    LiteralBool(bool),
    Identifier(Box<str>),
    Type(Vec<Spanned<Self>>),


    // Utility
    VarAndType(Box<Spanned<Self>>, Box<Spanned<Self>>),
    MutThis(bool),
    

    Unknown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    
    Or,
    And,

    Equals,
    NotEquals,
    Greater,
    GreaterOrEquals,
    Less,
    LessOrEquals,
}
