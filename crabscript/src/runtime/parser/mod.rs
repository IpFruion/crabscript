mod error;
use std::{collections::HashMap, rc::Rc, sync::Arc};

use clap::{CommandFactory, FromArgMatches};
use error::ParseInnerError;
pub use error::{ParseError, ParseErrorKind};
use miette::{NamedSource, SourceSpan};
use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::{
    command::{Command, CommandOutput},
    context::Context,
};

use super::{Directive, Instruction, Runtime};

#[derive(Parser)]
#[grammar = "crabshell.pest"]
struct CrabShellParser;

pub fn parse_runtime(ctx: Context, name: &str, src: String) -> Result<Runtime, ParseError> {
    let src: NamedSource<Arc<str>> = NamedSource::new(name, src.into()).with_language("crabshell");
    let mut pairs = match CrabShellParser::parse(Rule::program, src.inner()) {
        Ok(pairs) => pairs,
        Err(err) => {
            return Err(ParseErrorKind::Tokenizer(err.to_string().into())
                .into_inner_location(err.location)
                .into_parse_err(src))
        }
    };
    let program = pairs.next().expect("must have program");
    let mut directives = Vec::new();
    for prog_line in program.into_inner() {
        match prog_line.as_rule() {
            Rule::line => {
                if let Some(script_instruction) = prog_line.into_inner().next() {
                    let directive = match parse_directive(&ctx, script_instruction.into_inner()) {
                        Ok(directive) => directive,
                        Err(inner) => return Err(inner.into_parse_err(src)),
                    };
                    directives.push(directive);
                }
            }
            Rule::EOI => {
                break;
            }
            _ => unreachable!("if it parsed something it should be a directive"),
        }
    }

    Ok(Runtime::new(ctx, directives).with_src(src))
}

fn parse_directive(
    ctx: &Context,
    mut pairs: Pairs<'_, Rule>,
) -> Result<Directive, ParseInnerError> {
    let script_instruction = pairs.next().expect("script_instruction");
    match script_instruction.as_rule() {
        Rule::label => Ok(Directive::GoTo(script_instruction.as_str().to_string())),
        Rule::cmd_instruction => {
            let whole_cmd = script_instruction.as_span();
            let mut cmd = script_instruction.into_inner();
            let name = cmd.next().expect("cmd").as_span();
            let args: Vec<pest::Span> = cmd.map(|arg| arg.as_span()).collect();
            let parser = ctx.get_parser(name.as_str()).ok_or_else(|| {
                ParseErrorKind::CommandNotFound
                    .into_inner_spanned_with_reason(&name, "command not found")
            })?;
            let instruction = parser.0(whole_cmd, &args)?;
            Ok(Directive::Instruction {
                span: SourceSpan::new(
                    whole_cmd.start().into(),
                    whole_cmd.end() - whole_cmd.start(),
                ),
                instruction,
            })
        }
        _ => unreachable!("invalid script instruction"),
    }
}

#[derive(Clone)]
pub struct InstructionParser(Rc<InstructionParserFn>);

type InstructionParserFn =
    dyn Fn(pest::Span<'_>, &[pest::Span<'_>]) -> Result<Instruction, ParseInnerError>;

impl InstructionParser {
    pub fn insert_parser<T>(commands: &mut HashMap<Rc<str>, Self>)
    where
        T: CommandFactory + FromArgMatches + Command + 'static,
        T::Output: Into<CommandOutput> + 'static,
    {
        let cmd = T::command().no_binary_name(true);
        let name: Rc<str> = cmd.get_name().into();
        commands.entry(name).or_insert_with(|| {
            InstructionParser(Rc::new(move |cmd_span, args| {
                let matches = cmd
                    .clone()
                    .try_get_matches_from(args.iter().map(|span| span.as_str()))
                    .map_err(|err| ParseInnerError::from_clap_error(err, cmd_span, args))?;
                let t = T::from_arg_matches(&matches)
                    .map_err(|err| ParseInnerError::from_clap_error(err, cmd_span, args))?;
                Ok(Instruction::new_generic(t))
            }))
        });
        //TODO: Handle aliases
    }
}

#[cfg(test)]
mod tests {

    use crate::command::CommandError;

    use super::*;

    #[test]
    fn test_parse_empty() {
        parse_runtime(Default::default(), "parse_test", "".to_string()).unwrap();
    }

    #[test]
    fn test_parse_command_not_found() {
        let err = parse_runtime(
            Default::default(),
            "parse_test",
            "mkdir testing".to_string(),
        )
        .unwrap_err();

        assert!(
            matches!(err.kind, ParseErrorKind::CommandNotFound),
            "{:?}",
            err.kind
        );

        println!("{:?}", miette::Report::new(err));
    }

    #[derive(clap::Parser)]
    #[command(name = "tc")]
    struct TestCommand {
        arg1: String,
        arg2: u32,
        #[arg(required = false, long)]
        arg3: Option<String>,
    }

    impl Command for TestCommand {
        type Output = ();

        fn run(&self, _ctx: &mut Context) -> Result<Self::Output, CommandError> {
            println!("arg1: {}", self.arg1);

            Ok(())
        }
    }

    #[test]
    fn test_parse_command_test_cmd_missing_args() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let err = parse_runtime(context, "parse_test", "tc".to_string()).unwrap_err();

        assert!(
            matches!(err.kind, ParseErrorKind::InvalidArg(_)),
            "{:?}",
            err.kind
        );
        println!("{:?}", miette::Report::new(err));
    }

    #[test]
    fn test_parse_command_test_cmd_invalid_args() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let err = parse_runtime(context, "parse_test", "tc hello world".to_string()).unwrap_err();

        assert!(
            matches!(err.kind, ParseErrorKind::InvalidArg(_)),
            "{:?}",
            err.kind
        );
        println!("{:?}", miette::Report::new(err));
    }

    #[test]
    fn test_parse_command_test_cmd() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let runtime = parse_runtime(context, "parse_test", "tc hello 32".to_string()).unwrap();

        assert_eq!(runtime.directives.len(), 1);
    }

    #[test]
    fn test_parse_command_test_cmd_invalid_opt_arg() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let err =
            parse_runtime(context, "parse_test", "tc hello 32 --arg3".to_string()).unwrap_err();

        assert!(
            matches!(err.kind, ParseErrorKind::InvalidArg(_)),
            "{:?}",
            err.kind
        );
        println!("{:?}", miette::Report::new(err));
    }
}
