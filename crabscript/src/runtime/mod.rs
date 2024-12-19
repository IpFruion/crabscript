pub(crate) mod parser;
pub mod source;

use miette::{Diagnostic, LabeledSpan, SourceSpan};
pub use parser::{ParseError, ParseErrorKind};
use thiserror::Error;

use crate::{
    command::{Command, CommandError, CommandOutput},
    context::Context,
};
use source::SourceCode;

#[derive(Debug)]
pub struct Runtime {
    pub ctx: Context,
    pub directives: Vec<Directive>,
    src: Option<SourceCode>,
}

impl Runtime {
    pub fn new(ctx: Context, directives: Vec<Directive>) -> Self {
        Runtime {
            ctx,
            directives,
            src: None,
        }
    }

    pub fn parse(ctx: Context, src: SourceCode) -> Result<Self, ParseError> {
        parser::parse_runtime(ctx, &src).map(|mut runtime| {
            runtime.src = Some(src);
            runtime
        })
    }

    pub fn run(mut self) -> Result<(), RuntimeError> {
        for directive in self.directives {
            match directive {
                Directive::GoTo(_) => todo!(),
                Directive::Instruction {
                    span,
                    instruction,
                    output,
                    ..
                } => match instruction.run(&mut self.ctx) {
                    Ok(out) => {
                        if let Some(output) = output {
                            self.ctx.variables.insert(output, out);
                        }
                    }
                    Err(inner) => {
                        return Err(RuntimeError {
                            inner,
                            src: self.src,
                            span,
                        })
                    }
                },
            };
        }
        Ok(())
    }
}

pub enum Directive {
    GoTo(Box<str>),
    Instruction {
        label: Option<Box<str>>,
        span: SourceSpan,
        instruction: Instruction,
        output: Option<Box<str>>,
    },
}

impl std::fmt::Debug for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GoTo(arg0) => f.debug_tuple("GoTo").field(arg0).finish(),
            Self::Instruction { .. } => f.debug_tuple("Instruction").finish(),
        }
    }
}

pub struct Instruction(Box<dyn Command<Output = CommandOutput>>);

impl Instruction {
    pub fn new<C: Command<Output = CommandOutput> + 'static>(cmd: C) -> Self {
        Instruction(Box::new(cmd))
    }

    pub fn new_generic<C: Command + 'static>(cmd: C) -> Self
    where
        C::Output: Into<CommandOutput>,
    {
        Self::new(GenericInstruction::new(cmd))
    }

    #[inline]
    pub fn run(self, ctx: &mut Context) -> Result<CommandOutput, CommandError> {
        self.0.run(ctx)
    }
}

/// A generic instruction that uses [Into<CommandOutput>] for conversion of the [Command]
pub struct GenericInstruction<C>(C);

impl<T, C: Command<Output = T>> GenericInstruction<C> {
    pub fn new(cmd: C) -> Self {
        GenericInstruction(cmd)
    }
}

impl<T: Into<CommandOutput>, C: Command<Output = T>> Command for GenericInstruction<C> {
    type Output = CommandOutput;

    fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError> {
        Ok(self.0.run(ctx)?.into())
    }
}

#[derive(Error, Debug)]
#[error("error during runtime")]
pub struct RuntimeError {
    #[source]
    inner: CommandError,
    src: Option<SourceCode>,
    span: SourceSpan,
}

impl Diagnostic for RuntimeError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new("crabshell::runtime"))
    }
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.src.as_ref().map(|s| s as &dyn miette::SourceCode)
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(
            [LabeledSpan::new_with_span(
                self.inner.short_hand.clone().map(|s| s.to_string()),
                self.span,
            )]
            .into_iter(),
        ))
    }
}
