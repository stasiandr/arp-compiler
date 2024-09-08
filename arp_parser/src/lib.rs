use std::ops::Range;

use arp_lexer::tokens::ArpToken;
use arp_types::Spanned;

use chumsky::{prelude::*, Stream};
use errors::ParserError;
use types::ChumskyNode;


pub type Span = Range<usize>;

pub mod types;
pub mod tests;
pub mod errors;

pub mod atom;
pub mod expression;
pub mod statement;
pub mod declaration;


pub fn parse_arp_file<'a>(source_len: usize, input: &'a [arp_types::Spanned<ArpToken<'a>>]) -> Result<Spanned<ChumskyNode>, Vec<ParserError<'a>>> {
    parse(source_len, input, file_parser())
}


pub fn parse<'a>(source_len: usize, input: &'a [arp_types::Spanned<ArpToken<'a>>], parser : impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone) -> Result<Spanned<ChumskyNode>, Vec<ParserError<'a>>> {
    let stream = Stream::from_iter(
        source_len..source_len, 
        input.iter().map(|spanned| spanned.clone().destruct()));


    match parser.parse(stream) {
        Ok(node) => {
            Ok(node)
        },
        Err(err) => {
            Err(err.iter().map(|e| ParserError::SimpleError(e.clone())).collect())
        },
    }
}

fn file_parser<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let declaration = declaration::declaration();

    declaration
        .repeated()
        .then_ignore(end())
        .map(ChumskyNode::File)
        .map_with_span(Spanned::new)
}
