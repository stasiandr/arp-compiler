use std::{hash::Hash, num::ParseFloatError};

use logos::Logos;

use crate::errors::LexerError;


#[derive(Logos, Debug, PartialEq, Eq, Clone, Hash)]
#[logos(error = LexerError)]
#[logos(skip r#"[ \n\t\r\f]+"#)]
#[logos(skip r#"//[^\n]*\n?"#)]
#[logos(skip r#"/\*(?:[^*]|\*[^/])*\*/"#)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
pub enum ArpToken<'source> {
    #[token("from")] From,
    #[token("import")] Import,
    #[token("extern")] External,
    #[token("class")] Class,
    #[token("fn")] Function,
    #[token("union")] Union,
    #[token("struct")] Struct,
    #[token("impl")] Implementation,

    #[token("let")] Let,
    #[token("mut")] Mut,

    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While,
    #[token("for")] For,
    #[token("in")] In,
    #[token("break")] Break,
    #[token("return")] Return,
    

    #[token("this")] This,
    #[token("base")] Base,

    #[token("and")] And,
    #[token("or")] Or,


    #[token("==")] EqEq,
    #[token("!=")] BangEq,
    #[token(">")] Gt,
    #[token(">=")] GtEq,
    #[token("<")] Lt,
    #[token("<=")] LtEq,

    #[token("!")] Bang,
    #[token("=")] Eq,

    #[token(".")] Dot,
    #[token(",")] Comma,

    #[token(":")] Colon,
    #[token(";")] SemiColon,

    #[token("{")] BraceOpen,
    #[token("}")] BraceClose,

    #[token("(")] ParenthesisOpen,
    #[token(")")] ParenthesisClose,

    #[token("[")] BracketOpen,
    #[token("]")] BracketClose,

    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,

    #[token("->")] ThinArrow,

    #[regex(r#"[0-9][_0-9]*"#, |inp| inp.slice().parse().map_err(|err| (err, inp.span())))]
    Integer(i64),

    #[regex(r#"(?&decimal)(?:e(?&decimal)|\.(?&decimal)(?:e(?&decimal))?)"#, |inp| Float::parse(inp.slice()).map_err(|err| (err, inp.span())))]
    Float(Float),

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice())]
    String(&'source str),

    #[regex(r#"[\p{XID_Start}_]\p{XID_Continue}*"#)]
    Identifier(&'source str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Float(pub f64);

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_ne_bytes().hash(state);
    }
}

impl Float {
    #[inline]
    pub fn parse(s : &str) -> Result<Self, ParseFloatError> {
        match s.parse::<f64>() {
            Ok(ok) => Ok(ok.into()),
            Err(err) => Err(err),
        }
    }
}

impl Eq for Float { }

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Float> for f64 {
    fn from(val: Float) -> Self {
        val.0
    }
}