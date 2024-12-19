use std::error::Error;

use clap::error::{ContextKind, ErrorKind};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::runtime::source::SourceCode;

use super::Span;

/// An `Error` that implements [miette::Diagnostic] used in displaying where the error occured in
/// the source during parsing.
///
/// For determining the error `kind` see [ParseErrorKind]
#[derive(Error, Debug, Diagnostic)]
#[error("{kind}")]
#[diagnostic(code(crabshell::runtime::parser))]
pub struct ParseError {
    /// The kind of error that occured during parsing
    pub kind: ParseErrorKind,

    #[source_code]
    src: SourceCode,

    #[label("{reason}")]
    span: SourceSpan,

    reason: Box<str>,
}

/// The kind of error that occured in the parsing of the source script
#[derive(Error, Debug)]
pub enum ParseErrorKind {
    /// The command was not found in the [Context] during parsing
    #[error("Command not found inside Context")]
    CommandNotFound,

    /// The tokenizing / syntax analysis produced an error in the source code
    #[error("Error during tokenization: {0}")]
    Tokenizer(Box<str>),

    /// The argument to a command is invalid
    #[error("Invalid argument: {0}")]
    InvalidArg(Box<str>),
}

pub(super) struct ParseErrorBuilder {
    kind: ParseErrorKind,
    span: Option<SourceSpan>,
    reason: Option<Box<str>>,
}

impl ParseErrorBuilder {
    pub fn new_tokenizer<S: Into<Box<str>>>(msg: S) -> Self {
        Self::new(ParseErrorKind::Tokenizer(msg.into()))
    }

    pub fn new(kind: ParseErrorKind) -> Self {
        ParseErrorBuilder {
            kind,
            span: Default::default(),
            reason: Default::default(),
        }
    }
    pub fn with_node(self, node: tree_sitter::Node<'_>) -> Self {
        self.with_span(SourceSpan::new(
            node.start_byte().into(),
            node.end_byte() - node.start_byte(),
        ))
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_reason<S: Into<Box<str>>>(mut self, reason: S) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self, src: SourceCode) -> ParseError {
        let len = src.len();
        ParseError {
            kind: self.kind,
            src,
            span: self.span.unwrap_or_else(|| SourceSpan::new(0.into(), len)),
            reason: self.reason.unwrap_or_default(),
        }
    }

    pub fn from_clap_error(err: clap::Error, cmd_span: Span<'_>, args: &[Span<'_>]) -> Self {
        let builder = ParseErrorBuilder::new(ParseErrorKind::InvalidArg(err.to_string().into()));
        match err.kind() {
            ErrorKind::MissingRequiredArgument => {
                let reason = err
                    .get(ContextKind::InvalidArg)
                    .expect("missing arg")
                    .to_string();
                builder.with_span(cmd_span.into()).with_reason(reason)
            }
            ErrorKind::InvalidValue => {
                let span = clap_err_arg_span(&err, args).unwrap_or(&cmd_span);
                builder.with_span(span.into()).with_reason("invalid value")
            }
            ErrorKind::ValueValidation => {
                let reason = err.source().map(|err| err.to_string()).unwrap_or_default();
                let span = clap_err_arg_span(&err, args).unwrap_or(&cmd_span);
                builder.with_span(span.into()).with_reason(reason)
            }
            _ => builder.with_span(cmd_span.into()),
        }
    }
}

fn clap_err_arg_span<'a>(err: &clap::Error, args: &'a [Span<'a>]) -> Option<&'a Span<'a>> {
    let invalid_value = err
        .get(ContextKind::InvalidArg)
        .expect("invalid value")
        .to_string();
    let v = invalid_value
        .split_once(' ')
        .map(|(_, s)| s)
        .unwrap_or(&invalid_value);
    v.strip_prefix('<')
        .and_then(|v| v.strip_suffix('>'))
        .and_then(|s| s.split_once("ARG"))
        .and_then(|(_, s)| s.parse::<usize>().ok())
        .and_then(|index| args.get(index - 1))
}
