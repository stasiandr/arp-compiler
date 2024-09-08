use arp_lexer::tokens::ArpToken;
use arp_types::Spanned;
use chumsky::prelude::*;

use crate::{atom::{self}, types::{BinaryOp, ChumskyNode, UnaryOp}};

pub fn expr_parser<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    expr().then_ignore(end())
}

pub(crate) fn expr<'a>() -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let expr = recursive(|expr| {
        
        
        let ty = atom::ty();
        let ident = atom::ident();

        let call = ident.clone().map(Box::new)
            .then(expr.clone()
                .separated_by(just(ArpToken::Comma))
                .allow_trailing()
                .delimited_by(just(ArpToken::ParenthesisOpen), just(ArpToken::ParenthesisClose)))
            .map(|(call_ident, args)| ChumskyNode::CallExpr(call_ident, args))
            .map_with_span(Spanned::new);


        let construct = ty.map(Box::new)
            .then(((ident.clone().then_ignore(just(ArpToken::Colon))).or_not().then(expr.clone()))
                .separated_by(just(ArpToken::Comma))
                .allow_trailing()
                .delimited_by(just(ArpToken::BraceOpen), just(ArpToken::BraceClose)))
            .map(|(ident, args)| ChumskyNode::ConstructExpr(ident, args))
            .map_with_span(Spanned::new);


        let array = expr.clone()
            .separated_by(just(ArpToken::Comma))
            .allow_trailing()
            .delimited_by(just(ArpToken::BracketOpen), just(ArpToken::BracketClose))
            .map(ChumskyNode::ArrayExpr)
            .map_with_span(Spanned::new);

        let grouping = expr
            .delimited_by(just(ArpToken::ParenthesisOpen), just(ArpToken::ParenthesisClose));

        let atom = 
            atom::atom()
            .or(grouping)
            .or(construct)
            .or(call)
            .or(array)
            .or(ident);

        let get = atom.clone()
            .then(
                just(ArpToken::Dot)
                .then(atom.clone())
                .repeated())
            .foldl(|lhs, (_, rhs)| {
                let new_span = lhs.concat(&rhs);
                (ChumskyNode::GetExpr(Box::new(lhs), Box::new(rhs)), new_span).into()
            });

        let unary = 
            just(ArpToken::Minus).to(UnaryOp::Negate)
            .or(just(ArpToken::Bang).to(UnaryOp::Not)).map_with_span(|i, s|(i, s))
            .repeated()
            .then(get)
            .foldr(|(op, op_span), rhs| {
                let new_span = rhs.append_span(&op_span);
                (ChumskyNode::UnaryExpr(op, Box::new(rhs)), new_span).into()
            });
    
        precedence_climber(unary)
    });

    expr
}

fn precedence_climber<'a, U : Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone>(unary : U) -> impl Parser<ArpToken<'a>, Spanned<ChumskyNode>, Error = Simple<ArpToken<'a>>> + Clone {
    let logic_or = unary.clone()
        .then(
            just(ArpToken::Or).to(BinaryOp::Or)
            .then(unary)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        });

    let logic_and = logic_or.clone()
        .then(
            just(ArpToken::And).to(BinaryOp::And)
            .then(logic_or)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        });

    let equality = logic_and.clone()
        .then(
            just(ArpToken::EqEq).to(BinaryOp::Equals)
            .or(just(ArpToken::BangEq).to(BinaryOp::NotEquals))
            .then(logic_and)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        });

    let comparison = equality.clone()
        .then(
            just(ArpToken::Gt).to(BinaryOp::Greater)
            .or(just(ArpToken::GtEq).to(BinaryOp::GreaterOrEquals))
            .or(just(ArpToken::Lt).to(BinaryOp::Less))
            .or(just(ArpToken::LtEq).to(BinaryOp::LessOrEquals))
            .then(equality)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        });

    let product = comparison.clone()
        .then(
            just(ArpToken::Star).to(BinaryOp::Multiply)
            .or(just(ArpToken::Slash).to(BinaryOp::Divide))
            .then(comparison)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        });


    product.clone()
        .then(
            just(ArpToken::Plus).to(BinaryOp::Add)
            .or(just(ArpToken::Minus).to(BinaryOp::Subtract))
            .then(product)
            .repeated())
        .foldl(|lhs, (op, rhs)| {
            let new_span = lhs.concat(&rhs);
            (ChumskyNode::BinaryExpr(Box::new(lhs), op, Box::new(rhs)), new_span).into()
        })
}
