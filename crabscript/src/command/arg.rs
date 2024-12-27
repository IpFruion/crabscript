use core::str;
use std::{borrow::Cow, ops::Deref, str::FromStr};

use nom::IResult;
use thiserror::Error;

use crate::context::{Context, VariableResolver};

#[derive(Clone)]
pub enum Arg<T> {
    Value(T),
    Variable(Box<str>),
}

impl<'a, T: VariableResolver> Arg<T> {
    pub fn resolve(&'a self, ctx: &'a Context) -> Result<Cow<'a, T>, ArgNotFoundErr> {
        match self {
            Arg::Value(v) => Ok(Cow::Borrowed(v)),
            Arg::Variable(v) => ctx
                .variables
                .get(v)
                .map(T::resolve)
                .ok_or_else(|| ArgNotFoundErr(v.clone())),
        }
    }
}

impl<T: FromStr> FromStr for Arg<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = ArgParseError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix('$') {
            let (_, name) = parse_variable_name(stripped)
                .map_err(|err| ArgParseError::InvalidVariable(err.to_string().into()))?;
            return Ok(Arg::Variable(name.into()));
        }
        s.parse().map(Arg::Value).map_err(ArgParseError::Value)
    }
}

#[derive(Error, Debug)]
pub enum ArgParseError<E> {
    #[error("Error during parsing value {0}")]
    Value(E),
    #[error("Invalid variable {0}")]
    InvalidVariable(Box<dyn std::error::Error + Send + Sync>),
}

fn parse_variable_name(s: &str) -> IResult<&[u8], &str> {
    nom::combinator::map(
        nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            nom::bytes::complete::take_while1(|c| nom::character::is_alphanumeric(c) || c == b'_'),
            nom::bytes::complete::tag("}"),
        ),
        |s| {
            // # Safety Input to this function is a str so the contents should be
            unsafe { str::from_utf8_unchecked(s) }
        },
    )(s.as_bytes())
}

pub enum ArgValue<'a, T> {
    Value(&'a T),
    Variable(T),
}

impl<'a, T> Deref for ArgValue<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ArgValue::Value(v) => v,
            ArgValue::Variable(v) => v,
        }
    }
}

#[derive(Error, Debug)]
#[error("arg variable not found {0}")]
pub struct ArgNotFoundErr(Box<str>);

impl ArgNotFoundErr {
    pub fn arg(&self) -> &str {
        &self.0
    }
}
