use arp_lexer::tokens::ArpToken;
use arp_types::Spanned;
use chumsky::prelude::*;

use crate::types::ChumskyNode;

pub fn atom_parser<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    atom().then_ignore(end())
}

pub(crate) fn atom<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let atom = select! {
        ArpToken::This => ChumskyNode::This,
        ArpToken::Base => ChumskyNode::Base,
        ArpToken::Integer(i) => ChumskyNode::LiteralInteger(i),
        ArpToken::Bool(b) => ChumskyNode::LiteralBool(b),
        ArpToken::String(s) => ChumskyNode::LiteralString(s.into()),
        ArpToken::Float(f) => ChumskyNode::LiteralFloat(f),
    };

    atom.map_with_span(Spanned::new)
}

pub(crate) fn ident<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    select! {ArpToken::Identifier(ident) => ChumskyNode::Identifier(Box::from(ident))}
        .map(|i| i)
        .map_with_span(Spanned::new)
}

pub(crate) fn ty<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let ident = ident();

    ident
        .separated_by(just(ArpToken::Dot))
        .map(ChumskyNode::Type)
        .map_with_span(Spanned::new)
}