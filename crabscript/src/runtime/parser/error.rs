use std::{error::Error, sync::Arc};

use clap::error::{ContextKind, ErrorKind};
use miette::{Diagnostic, NamedSource, SourceSpan};
use pest::error::InputLocation;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{kind}")]
#[diagnostic(code(crabshell::runtime::parser))]
pub struct ParseError {
    pub kind: ParseErrorKind,

    #[source_code]
    src: NamedSource<Arc<str>>,

    #[label("{reason}")]
    span: SourceSpan,

    reason: Box<str>,
}

pub(super) struct ParseInnerError {
    kind: ParseErrorKind,
    span: SourceSpan,
    reason: Option<Box<str>>,
}

impl ParseInnerError {
    pub(super) fn into_parse_err(self, src: NamedSource<Arc<str>>) -> ParseError {
        ParseError {
            kind: self.kind,
            src,
            span: self.span,
            reason: self.reason.map(Box::from).unwrap_or_default(),
        }
    }

    pub(super) fn from_clap_error(
        err: clap::Error,
        cmd_span: pest::Span<'_>,
        args: &[pest::Span<'_>],
    ) -> Self {
        let err_kind = ParseErrorKind::InvalidArg(err.to_string().into());
        match err.kind() {
            ErrorKind::MissingRequiredArgument => {
                let reason = err
                    .get(ContextKind::InvalidArg)
                    .expect("missing arg")
                    .to_string();
                err_kind.into_inner_spanned_with_reason(&cmd_span, reason)
            }
            ErrorKind::InvalidValue => {
                let span = clap_err_arg_span(&err, args).unwrap_or(&cmd_span);
                err_kind.into_inner_spanned_with_reason(span, "invalid value")
            }
            ErrorKind::ValueValidation => {
                let reason = err.source().map(|err| err.to_string()).unwrap_or_default();
                let span = clap_err_arg_span(&err, args).unwrap_or(&cmd_span);
                err_kind.into_inner_spanned_with_reason(span, reason)
            }
            _ => err_kind.into_inner_spanned(&cmd_span),
        }
    }
}

fn clap_err_arg_span<'a>(
    err: &clap::Error,
    args: &'a [pest::Span<'a>],
) -> Option<&'a pest::Span<'a>> {
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

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Command not found inside Context")]
    CommandNotFound,

    #[error("Error during tokenization: {0}")]
    Tokenizer(Box<str>),

    #[error("Invalid argument: {0}")]
    InvalidArg(Box<str>),
}

impl ParseErrorKind {
    pub(super) fn into_inner_location(self, location: InputLocation) -> ParseInnerError {
        ParseInnerError {
            kind: self,
            reason: None,
            span: match location {
                InputLocation::Pos(s) => SourceSpan::new(0.into(), s),
                InputLocation::Span((a, b)) => SourceSpan::new(a.into(), b - a),
            },
        }
    }

    pub(super) fn into_inner_spanned(self, span: &pest::Span<'_>) -> ParseInnerError {
        ParseInnerError {
            kind: self,
            reason: None,
            span: SourceSpan::new(span.start().into(), span.end() - span.start()),
        }
    }

    pub(super) fn into_inner_spanned_with_reason<R: Into<Box<str>>>(
        self,
        span: &pest::Span<'_>,
        reason: R,
    ) -> ParseInnerError {
        ParseInnerError {
            kind: self,
            reason: Some(reason.into()),
            span: SourceSpan::new(span.start().into(), span.end() - span.start()),
        }
    }
}
