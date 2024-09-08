use arp_lexer::tokens::ArpToken;
use arp_types::Spanned;
use chumsky::prelude::*;

use crate::{atom, expression, types::ChumskyNode};

pub fn statement_parser<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    statement().then_ignore(end())
}

pub(crate) fn rec_statement<'a>(decl: Recursive<'a, ArpToken<'a>, Spanned<ChumskyNode>, Simple<ArpToken<'a>>>) -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone + 'a {
    let expression = expression::expr();
    let block_parser = decl.repeated()
        .then(expression.or_not())
        .delimited_by(just(ArpToken::BraceOpen), just(ArpToken::BraceClose))
        .map(|(v, ret_expr)| ChumskyNode::BlockStmt(v, ret_expr.map(|r| r.into())))
        .map_with_span(Spanned::new);

    let statement = statement();

    block_parser
        .or(statement)
}

pub(crate) fn statement<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let statement = recursive(|stmt| {
        let inline_expr = expression::expr();

        let ident = atom::ident();

        let ty = atom::ty();

        let variable_decl = just(ArpToken::Let)
            .ignore_then(just(ArpToken::Mut).or_not())
            .then(ident.clone())
            .then(just(ArpToken::Colon).ignore_then(ty.clone()).or_not())
            .then_ignore(just(ArpToken::Eq))
            .then(inline_expr.clone())
            .then_ignore(just(ArpToken::SemiColon))
            .map(|(((r#mut, ident), ty), expr)| ChumskyNode::VariableDecl(r#mut.is_some(), ident.into(), ty.map(|t| t.into()), expr.into()))
            .map_with_span(Spanned::new);


        let block_parser = stmt
            .repeated()
            .then(inline_expr.clone().or_not())
            .delimited_by(just(ArpToken::BraceOpen), just(ArpToken::BraceClose))
            .map(|(v, ret_expr)| ChumskyNode::BlockStmt(v, ret_expr.map(|r| r.into())))
            .map_with_span(Spanned::new);
        
        let expr_stmt = inline_expr.clone()
            .then_ignore(just(ArpToken::SemiColon))
            .map(|node| ChumskyNode::ExpressionStmt(node.into()))
            .map_with_span(Spanned::new);
    
        let assi_stmt = inline_expr.clone()
            .then_ignore(just(ArpToken::Eq))
            .then(inline_expr.clone())
            .then_ignore(just(ArpToken::SemiColon))
            .map(|(lhs, rhs)| ChumskyNode::AssignmentStmt(lhs.into(), rhs.into()))
            .map_with_span(Spanned::new);
    
        let if_stmt = recursive(|if_stmt: Recursive<ArpToken, Spanned<ChumskyNode>, Simple<ArpToken>>| {
            just(ArpToken::If)
                .ignore_then(inline_expr.clone())
                .then(block_parser.clone())
                .then(
                    just(ArpToken::Else)
                    .ignore_then(just(ArpToken::If))
                    .ignore_then(inline_expr.clone())
                    .then(block_parser.clone())
                    .repeated()
                )
                .then(
                    just(ArpToken::Else)
                    .ignore_then(block_parser.clone().or(if_stmt))
                    .or_not())
                .map(|(((cond, a), else_if), b)| {
                    ChumskyNode::IfStmt(cond.into(), a.into(), else_if, b.map(|n| n.into()))
                }).map_with_span(Spanned::new)
        });
    
        let while_stmt = just(ArpToken::While)
            .ignore_then(inline_expr.clone())
            .then(block_parser.clone())
            .map(|(cond, block)| ChumskyNode::WhileStmt(cond.into(), block.into()))
            .map_with_span(Spanned::new);

        let for_stmt = just(ArpToken::For)
            .ignore_then(ident.clone())
            .then_ignore(just(ArpToken::In))
            .then(ident.clone())
            .then(block_parser.clone())
            .map(|((ident, ident_iter), block)| ChumskyNode::ForStmt(ident.into(), ident_iter.into(), block.into()))
            .map_with_span(Spanned::new);
    
        let return_stmt = just(ArpToken::Return)
            .ignore_then(inline_expr.clone())
            .then_ignore(just(ArpToken::SemiColon))
            .map(|ex| ChumskyNode::ReturnStmt(ex.into()))
            .map_with_span(Spanned::new);

        let break_stmt = just(ArpToken::Break)
            .ignore_then(just(ArpToken::SemiColon))
            .map(|_| ChumskyNode::Break)
            .map_with_span(Spanned::new);
    
    
        expr_stmt
            .or(assi_stmt)
            .or(variable_decl)
            .or(block_parser)
            .or(return_stmt)
            .or(break_stmt)
            .or(while_stmt)
            .or(for_stmt)
            .or(if_stmt)
    });

    statement
}
