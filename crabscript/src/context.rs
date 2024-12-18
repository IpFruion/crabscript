use std::{collections::HashMap, env::current_dir, path::PathBuf, rc::Rc};

use clap::{CommandFactory, FromArgMatches};

use crate::{
    command::{Command, CommandOutput},
    runtime::parser::InstructionParser,
    sdk::echo::Echo,
};

pub struct Context {
    pub cur_dir: Option<PathBuf>,
    pub variables: HashMap<Box<str>, CommandOutput>,
    parsers: HashMap<Rc<str>, InstructionParser>,
}

impl Context {
    pub fn init() -> Self {
        let mut ctx = Context {
            cur_dir: current_dir().ok(),
            ..Default::default()
        };
        ctx.add_command::<Echo>();
        ctx
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("cur_dir", &self.cur_dir)
            .field("variables", &self.variables)
            .finish()
    }
}

/// Creates an empty [Context] with zero internal commands, variables, and empty current directory.
impl Default for Context {
    fn default() -> Self {
        Self {
            cur_dir: None,
            variables: Default::default(),
            parsers: Default::default(),
        }
    }
}

impl Context {
    #[inline]
    pub fn add_command<T>(&mut self)
    where
        T: CommandFactory + FromArgMatches + Command + 'static,
        T::Output: Into<CommandOutput> + 'static,
    {
        InstructionParser::insert_parser::<T>(&mut self.parsers);
    }

    pub fn get_parser(&self, name: &str) -> Option<&InstructionParser> {
        self.parsers.get(name)
    }
}
