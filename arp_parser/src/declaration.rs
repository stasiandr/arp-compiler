use arp_lexer::tokens::ArpToken;
use arp_types::Spanned;
use chumsky::prelude::*;

use crate::{atom, statement, types::ChumskyNode};

pub fn declaration_parser<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    declaration().then_ignore(end())
}

pub(crate) fn declaration<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let decl = recursive(|decl| {

        let statement = statement::rec_statement(decl);
    
        let ident = atom::ident();
        let ty = atom::ty();
    
        let statement_decl = statement.clone()
            .map(Box::new)
            .map(ChumskyNode::StatementDecl)
            .map_with_span(Spanned::new);
        
    
        let function_decl = just(ArpToken::Function)
            .ignore_then(ident.clone())
            .then(
                (ident.clone()
                    .then_ignore(just(ArpToken::Colon))
                    .then(ty.clone()))
                    .map_with_span(|pair, s| Into::<Spanned<ChumskyNode>>::into((ChumskyNode::VarAndType(pair.0.into(), pair.1.into()), s)))
                .or(just(ArpToken::Mut).or_not().then_ignore(just(ArpToken::This)).map_with_span(|s, span| (ChumskyNode::MutThis(s.is_some()), span).into()))
                .separated_by(just(ArpToken::Comma))
                .allow_trailing()
                .delimited_by(just(ArpToken::ParenthesisOpen), just(ArpToken::ParenthesisClose)))
            .then(
                just(ArpToken::ThinArrow)
                .ignore_then(ty.clone())
                .or_not())
            .then(statement.clone())
            .map(|(((ident, args), return_type), block)| ChumskyNode::FuncDecl(ident.into(), args, return_type.map(|r| r.into()), block.into()))
            .map_with_span(Spanned::new);
    
        let class_declaration = just(ArpToken::Class)
            .ignore_then(ty.clone())
            .then(
                just(ArpToken::Colon)
                .ignore_then(ty.clone().separated_by(just(ArpToken::Comma))).or_not()
            )
            .then(
                (ident.clone()
                    .then_ignore(just(ArpToken::Colon))
                    .then(ty.clone()))
                    .map_with_span(|pair, s| Into::<Spanned<ChumskyNode>>::into((ChumskyNode::VarAndType(pair.0.into(), pair.1.into()), s)))
                .separated_by(just(ArpToken::Comma))
                .allow_trailing()
                .delimited_by(just(ArpToken::BraceOpen), just(ArpToken::BraceClose)))
            .map(|((ident, extends), fields)| ChumskyNode::Structure(ident.into(), extends.unwrap_or_default(), fields))
            .map_with_span(Spanned::new);
    
        let implementation_declaration = just(ArpToken::Implementation)
            .ignore_then(ty.clone())
            .then(
                function_decl.clone()
                .repeated()
                .delimited_by(just(ArpToken::BraceOpen), just(ArpToken::BraceClose)))
            .map(|(ident, funcs)| ChumskyNode::ImplementationDecl(ident.into(), funcs))
            .map_with_span(Spanned::new);
    
        let import_declaration = just(ArpToken::From)
            .ignore_then(just(ArpToken::External).or_not())
            .then(ident.clone().separated_by(just(ArpToken::Dot)))
            .then_ignore(just(ArpToken::Import))
            .then(
                ty
                .separated_by(just(ArpToken::Comma))
                .allow_trailing())
            .map(|((ext, expr), idents)| ChumskyNode::ImportDecl(ext.is_some(), expr, idents))
            .map_with_span(Spanned::new);
    
        statement_decl
            .or(function_decl)
            .or(class_declaration)
            .or(implementation_declaration)
            .or(import_declaration)
    });

    decl
}
