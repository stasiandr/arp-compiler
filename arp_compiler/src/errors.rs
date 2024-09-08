use std::io;

use arp_lexer::errors::LexerError;
use arp_parser::errors::ParserError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("{0}")]
    Parser(#[from] Box<ParserError<'static>>),

    #[error("{0}")]
    Lexer(#[from] LexerError),

    #[error("{0}")]
    IO(#[from] io::Error),

    #[error("Can't deserialize toml")]
    TomlError(#[from] toml::de::Error),

    #[error("Custom")]
    Custom(String),
}
