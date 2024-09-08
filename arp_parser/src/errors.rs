
use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind};
use arp_lexer::tokens::ArpToken;
use arp_types::{errors::DiagnosticError, sources::Source, traits::{AppendToReport, RB}};
use chumsky::error::{Simple, SimpleReason};
use thiserror::Error;



#[derive(Error, Debug, PartialEq, Eq, Clone, Default)]
pub enum ParserError<'a> {

    #[error("Simple parser error")]
    SimpleError(Simple<ArpToken<'a>, Range<usize>>),

    #[error("Unknown")]
    #[default]
    Unknown,
}


impl<'a> AppendToReport<Self> for ParserError<'a> {
    fn append_to_report<'source>(&self, builder: RB<'source>, source: &'source Source) -> RB<'source> {
        match self {
            ParserError::SimpleError(error) => {
                builder
                    .with_label(
                        Label::new((source.get_path_string(), error.span()))
                            .with_message(format!("Reason: {:?}", error.reason()))
                            .with_color(Color::Red)
                            .with_message(format!("{:?}", error))
                            .with_color(Color::White)
                            
                    )
            },
            Self::Unknown => builder,
            
        }
    }

    fn build_report<'source>(errors: &'source [Self], source: &'source Source) -> Report<'source, (String, Range<usize>)> where Self: Sized {
        let mut builder: RB = Report::build(ReportKind::Error, source.get_path_string(), 0)
            .with_message("Parser Error".to_owned());

        for error in errors {
            builder = error.append_to_report(builder, source);
        }

        builder.finish()
    }
}




impl<'a> TryFrom<ParserError<'a>> for DiagnosticError {
    type Error = ParserError<'a>;

    fn try_from(value: ParserError<'a>) -> Result<Self, Self::Error> {
        match value {
            ParserError::SimpleError(token) => {
                match token.reason() {
                    SimpleReason::Unexpected => {
                        Ok(DiagnosticError::new(token.span(), format!("Found {:?}. Expected on of: {:?}", token.found(), token.expected().collect::<Vec<_>>())))
                    }
                    SimpleReason::Unclosed { span, delimiter } => {
                        Ok(DiagnosticError::new(span.clone(), format!("Unclosed delimiter: {delimiter:?}")))
                    },
                    SimpleReason::Custom(custom) => Ok(DiagnosticError::new(token.span(), custom.as_str())),
                }
            },
            ParserError::Unknown => Err(value),
        }
    }
}
