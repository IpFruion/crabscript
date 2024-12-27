pub mod directive;
pub mod instruction;
pub(crate) mod parser;
pub mod source;

use directive::Directive;
use miette::{Diagnostic, LabeledSpan, SourceSpan};
pub use parser::{ParseError, ParseErrorKind};
use thiserror::Error;

use crate::{
    command::{CommandError, CommandOutput},
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
                Directive::Instruction {
                    span,
                    instruction,
                    output,
                    ..
                } => match instruction.run(&mut self.ctx) {
                    Ok(out) => {
                        if let Some(output) = output {
                            if let CommandOutput::VariableValue(v) = out {
                                self.ctx.variables.insert(output, v);
                            }
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
                _ => todo!(),
            };
        }
        Ok(())
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
