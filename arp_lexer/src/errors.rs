use std::num::{ParseFloatError, ParseIntError};
use std::ops::Range;
use std::str::ParseBoolError;

use ariadne::{Color, ReportKind};
use ariadne::{Label, Report};
use arp_types::errors::DiagnosticError;
use arp_types::sources::Source;
use arp_types::traits::{AppendToReport, RB};
use thiserror::Error;


#[derive(Error, Debug, PartialEq, Eq, Clone, Default)]
pub enum LexerError {

    #[error("Unrecognized")]
    ConvertError(Range<usize>, ConverterErrorKind),

    #[error("Unrecognized")]
    Unrecognized(Range<usize>),

    #[error("Unknown")]
    #[default]
    Unknown,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Default)]
pub enum ConverterErrorKind {

    #[error("Bool")]
    Bool,
    
    #[error("Float")]
    Float,

    #[error("Int")]
    Int,

    #[error("Unknown")]
    #[default]
    Unknown,
}

impl From<(ParseIntError, Range<usize>)> for LexerError {
    #[inline]
    fn from(value: (ParseIntError, Range<usize>)) -> Self {
        Self::ConvertError(value.1, ConverterErrorKind::Int)
    }
}

impl From<(ParseFloatError, Range<usize>)> for LexerError {
    #[inline]
    fn from(value: (ParseFloatError, Range<usize>)) -> Self {
        Self::ConvertError(value.1, ConverterErrorKind::Float)
    }
}

impl From<(ParseBoolError, Range<usize>)> for LexerError {
    #[inline]
    fn from(value: (ParseBoolError, Range<usize>)) -> Self {
        Self::ConvertError(value.1, ConverterErrorKind::Bool)
    }
}

impl AppendToReport<Self> for LexerError {
    fn append_to_report<'source>(&self, builder: RB<'source>, source: &'source Source) -> RB<'source> {
        match self {
            Self::Unrecognized(span) => {
                builder
                    .with_label(
                        Label::new((source.get_path_string(), span.clone()))
                            .with_message("Unrecognized token")
                            .with_color(Color::Red)
                    )
            },
            Self::ConvertError(span, kind) => {
                builder
                    .with_label(
                        Label::new((source.get_path_string(), span.clone()))
                            .with_message(format!("Can't parse token of a kind: {kind}"))
                            .with_color(Color::Red)
                    )
            },
            Self::Unknown => builder,
        }
    }
    
    fn build_report<'source>(errors: &'source [Self], source: &'source Source) -> Report<'source, (String, Range<usize>)> where Self: Sized {
        let mut builder: RB = Report::build(ReportKind::Error, source.get_path_string(), 0)
            .with_message("Lexer Error".to_owned());

        for error in errors {
            builder = error.append_to_report(builder, source);
        }

        builder.finish()
    }
}


impl TryInto<DiagnosticError> for LexerError {
    type Error = LexerError;
    
    fn try_into(self) -> Result<DiagnosticError, Self::Error> {
        match self {
            LexerError::ConvertError(range, kind) => {
                let message = match kind {
                    ConverterErrorKind::Bool => "Cant' parse bool",
                    ConverterErrorKind::Float => "Cant' parse float",
                    ConverterErrorKind::Int => "Cant' parse integer",
                    ConverterErrorKind::Unknown => "Can't parse unknown",
                };

                Ok(DiagnosticError::new(range, message))
            },
            LexerError::Unrecognized(range) => {
                Ok(DiagnosticError::new(range, "Unrecognized token found"))
            },
            LexerError::Unknown => Err(LexerError::Unknown),
        }
    }
}