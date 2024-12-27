use std::{collections::HashMap, fmt::Debug, ops::Deref, rc::Rc};

use clap::{CommandFactory, FromArgMatches};

use crate::{
    command::{Command, CommandOutput},
    runtime::{
        instruction::{GenericInstruction, Instruction},
        ParseErrorKind,
    },
    sdk::{echo::Echo, set::Set},
};

use super::{
    error::ParseErrorBuilder,
    tokenizer::{Cursor, Field},
    Span,
};

/// Holds onto the contents of the commands to be parsed
#[derive(Default)]
pub struct InstructionParsers {
    parsers: HashMap<Box<str>, Rc<InstructionParser>>,
}

impl InstructionParsers {
    /// Adds the default commands including ones defined by the sdk
    pub fn init() -> Self {
        let mut parsers = InstructionParsers::default();
        parsers.add_generic::<Echo, _>(InstructionParser::Echo);
        parsers.add_generic::<Set, _>(InstructionParser::Set);
        parsers
    }

    fn add_generic<T, F>(&mut self, instruction_parser: F)
    where
        T: CommandFactory + Command,
        T::Output: Into<CommandOutput>,
        F: FnOnce(clap::Command) -> InstructionParser,
    {
        let cmd = T::command().no_binary_name(true);
        let name: Box<str> = cmd.get_name().into();
        let parser = Rc::new(instruction_parser(cmd));
        self.parsers.insert(name, parser);
    }

    #[inline]
    pub fn add_parser<T>(&mut self)
    where
        T: CommandFactory + FromArgMatches + Command + 'static,
        T::Output: Into<CommandOutput>,
    {
        self.add_generic::<T, _>(InstructionParser::new_generic::<T>)
    }

    pub(super) fn parse(&self, cursor: &Cursor<'_>) -> Result<Instruction, ParseErrorBuilder> {
        let cmd = cursor.child_by_field(Field::Cmd)?.span();
        let args: Vec<_> = cursor
            .children_by_field(Field::Arg)?
            .into_iter()
            .map(|cursor| cursor.span())
            .collect();
        self.parsers
            .get(cmd.deref())
            .map(|s| s.as_ref())
            .ok_or_else(|| {
                ParseErrorBuilder::new(ParseErrorKind::CommandNotFound)
                    .with_span(cmd.into())
                    .with_reason("command not found")
            })?
            .parse(cmd, &args)
    }
}

impl Debug for InstructionParsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstructionParser")
            .field("commands", &self.parsers.keys().collect::<Vec<_>>())
            .finish()
    }
}

pub enum InstructionParser {
    Echo(clap::Command),
    Set(clap::Command),
    Generic(Box<GenericParser>),
}

impl InstructionParser {
    pub fn new_generic<T>(cmd: clap::Command) -> Self
    where
        T: FromArgMatches + Command + 'static,
        T::Output: Into<CommandOutput>,
    {
        InstructionParser::Generic(Box::new(move |cmd_span, args| {
            parse::<T>(cmd.clone(), cmd_span, args)
                .map(|t| Box::new(GenericInstruction::new(t)) as _)
        }))
    }

    pub(super) fn parse(
        &self,
        cmd_span: Span<'_>,
        args: &[Span<'_>],
    ) -> Result<Instruction, ParseErrorBuilder> {
        match self {
            InstructionParser::Set(v) => parse::<Set>(v.clone(), cmd_span, args).map(Into::into),
            InstructionParser::Echo(v) => parse::<Echo>(v.clone(), cmd_span, args).map(Into::into),
            InstructionParser::Generic(g) => g(cmd_span, args).map(Into::into),
        }
    }
}

type GenericParser = dyn Fn(
    Span<'_>,
    &[Span<'_>],
) -> Result<Box<dyn Command<Output = CommandOutput>>, ParseErrorBuilder>;

fn parse<T>(
    cmd: clap::Command,
    cmd_span: Span<'_>,
    args: &[Span<'_>],
) -> Result<T, ParseErrorBuilder>
where
    T: FromArgMatches + Command,
{
    let matches = cmd
        .try_get_matches_from(args.iter().map(|span| span.deref()))
        .map_err(|err| ParseErrorBuilder::from_clap_error(err, cmd_span, args))?;

    T::from_arg_matches(&matches)
        .map_err(|err| ParseErrorBuilder::from_clap_error(err, cmd_span, args))
}
