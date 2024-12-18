use std::marker::PhantomData;

use clap::builder::{TypedValueParser, ValueParserFactory};
use thiserror::Error;

use crate::context::Context;

pub trait Command {
    type Output;

    fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError>;
}

#[derive(Debug)]
pub struct CommandOutput;

impl From<()> for CommandOutput {
    fn from(_value: ()) -> Self {
        CommandOutput
    }
}

#[derive(Error, Debug)]
#[error("{source}")]
pub struct CommandError {
    pub(crate) short_hand: Option<Box<str>>,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl CommandError {
    pub fn new<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        CommandError {
            source: Box::new(err),
            short_hand: None,
        }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::new(value)
    }
}

#[derive(Clone)]
pub enum Arg<T> {
    Value(T),
    Variable(Box<str>),
}

impl<T> ValueParserFactory for Arg<T> {
    type Parser = ArgValueParser<T>;

    fn value_parser() -> Self::Parser {
        ArgValueParser(Default::default())
    }
}

pub struct ArgValueParser<T>(PhantomData<T>);

impl<T> Clone for ArgValueParser<T> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<T: ValueParserFactory + Clone + Sync + Send + 'static> TypedValueParser for ArgValueParser<T> {
    type Value = Arg<T>;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        value.to_string_lossy();
        let _inner_parser = clap::value_parser!(T);
        todo!()
    }
}
