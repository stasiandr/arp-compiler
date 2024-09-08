pub mod tokens;
pub mod errors;

use arp_types::{sources::Source, Spanned};
use errors::LexerError;
use logos::Logos;
use tokens::ArpToken;

#[inline]
pub fn lex_tokens(source: &Source) -> Result<Vec<Spanned<ArpToken>>, Vec<LexerError>> {
    let lex = ArpToken::lexer(source.content());

    let mut tokens = vec![];
    let mut lexer_errors = vec![];

    for (token, span) in lex.spanned() {
        match token {
            Ok(token) => {
                tokens.push(Spanned::new(token, span));
            },
            Err(LexerError::Unknown) => {
                lexer_errors.push(LexerError::Unrecognized(span));
            },
            Err(err) => {
                lexer_errors.push(err);
            },
        }
    }

    if lexer_errors.is_empty() {
        Ok(tokens)
    } else {
        Err(lexer_errors)
    }
}