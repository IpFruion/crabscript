use std::{borrow::Cow, collections::HashMap, env::current_dir, path::PathBuf};

use clap::{CommandFactory, FromArgMatches};

use crate::{
    command::{Command, CommandOutput},
    runtime::parser::InstructionParsers,
};

#[derive(Debug, Default)]
pub struct Context {
    pub cur_dir: Option<PathBuf>,
    pub variables: HashMap<Box<str>, Variable>,
    parser: InstructionParsers,
}

impl Context {
    /// Initializes with environment and sdk commands if added in feature selection
    pub fn init<S: Into<String>, I: Iterator<Item = S>>(args: I) -> Self {
        let variables = args
            .enumerate()
            .map(|(i, v)| (i.to_string().into(), Variable::String(v.into())))
            .collect();
        Context {
            cur_dir: current_dir().ok(),
            variables,
            parser: InstructionParsers::init(),
        }
    }

    #[inline]
    pub fn add_command<T>(&mut self)
    where
        T: CommandFactory + FromArgMatches + Command + 'static,
        T::Output: Into<CommandOutput>,
    {
        self.parser.add_parser::<T>()
    }

    pub(crate) fn get_parser(&self) -> &InstructionParsers {
        &self.parser
    }
}

/// The Value of a variable
#[derive(Debug)]
pub enum Variable {
    String(String),
    Boolean(bool),
}

pub trait VariableResolver: ToOwned {
    fn resolve(variable: &Variable) -> Cow<'_, Self>;
}

impl VariableResolver for String {
    fn resolve(variable: &Variable) -> Cow<'_, Self> {
        match variable {
            Variable::String(s) => Cow::Borrowed(s),
            Variable::Boolean(b) => Cow::Owned(b.to_string()),
        }
    }
}

impl VariableResolver for Box<str> {
    fn resolve(variable: &Variable) -> Cow<'_, Self> {
        match variable {
            Variable::String(s) => Cow::Owned(Box::from(s.as_str())),
            Variable::Boolean(b) => Cow::Owned(b.to_string().into()),
        }
    }
}
