mod error;
use std::{collections::HashMap, rc::Rc, sync::Arc};

use clap::{CommandFactory, FromArgMatches};
use error::ParseInnerError;
pub use error::{ParseError, ParseErrorKind};
use miette::{NamedSource, SourceSpan};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{
    command::{Command, CommandOutput},
    context::Context,
};

use super::{Directive, Instruction, Runtime};

pub struct Span(SourceSpan);

impl From<&pest::Span<'_>> for Span {
    fn from(value: &pest::Span<'_>) -> Self {
        Span(SourceSpan::new(
            value.start().into(),
            value.end() - value.start(),
        ))
    }
}

#[derive(Parser)]
#[grammar = "crabscript.pest"]
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
    let mut label = None;
    let mut output = None;

    let mut piece = pairs.next().expect("at least one script_instruction");
    if matches!(piece.as_rule(), Rule::label) {
        let new_label = piece.into_inner().next().expect("ident").as_str().into();
        let Some(next_piece) = pairs.next() else {
            return Ok(Directive::GoTo(new_label));
        };
        label = Some(new_label);
        piece = next_piece;
    }
    if matches!(piece.as_rule(), Rule::output) {
        output = Some(piece.into_inner().next().expect("ident").as_str().into());
        piece = pairs.next().expect("at least cmd");
    }
    if matches!(piece.as_rule(), Rule::cmd_instruction) {
        let (name, whole_cmd, args) = parse_cmd_instruction(piece);
        let parser = ctx.get_parser(name.as_str()).ok_or_else(|| {
            ParseErrorKind::CommandNotFound
                .into_inner_spanned_with_reason(&name, "command not found")
        })?;
        let instruction = parser.0(whole_cmd, &args)?;
        return Ok(Directive::Instruction {
            label,
            span: SourceSpan::new(
                whole_cmd.start().into(),
                whole_cmd.end() - whole_cmd.start(),
            ),
            instruction,
            output,
        });
    }
    unreachable!("invalid script instruction")
}

fn parse_cmd_instruction(
    pair: Pair<'_, Rule>,
) -> (pest::Span<'_>, pest::Span<'_>, Vec<pest::Span<'_>>) {
    let whole_cmd = pair.as_span();
    let mut cmd_instruction = pair.into_inner();
    let name = cmd_instruction.next().expect("cmd").as_span();
    let args: Vec<pest::Span> = parse_args(cmd_instruction);
    (name, whole_cmd, args)
}

fn parse_args<'a>(args: impl Iterator<Item = Pair<'a, Rule>>) -> Vec<pest::Span<'a>> {
    args.map(|arg| arg.into_inner().next().expect("inner arg"))
        .map(|arg| match arg.as_rule() {
            Rule::arg_raw => arg.as_span(),
            Rule::arg_qouted => arg.as_span(),
            _ => unreachable!("Not handled"),
        })
        .collect()
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

    use std::ops::Deref;

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

    #[test]
    fn test_parse_command_output() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let runtime =
            parse_runtime(context, "parse_test", "var = tc hello 32".to_string()).unwrap();
        assert_eq!(runtime.directives.len(), 1);
        match &runtime.directives[0] {
            Directive::GoTo(_) => unreachable!(),
            Directive::Instruction { label, output, .. } => {
                assert!(label.is_none());
                assert_eq!(output.as_deref(), Some("var"));
            }
        }
    }

    #[test]
    fn test_parse_command_label() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let runtime =
            parse_runtime(context, "parse_test", ":label tc hello 32".to_string()).unwrap();
        assert_eq!(runtime.directives.len(), 1);
        match &runtime.directives[0] {
            Directive::GoTo(_) => unreachable!(),
            Directive::Instruction { label, output, .. } => {
                assert!(output.is_none());
                assert_eq!(label.as_deref(), Some("label"));
            }
        }
    }

    #[test]
    fn test_parse_goto() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let runtime = parse_runtime(context, "parse_test", ":label".to_string()).unwrap();
        assert_eq!(runtime.directives.len(), 1);
        assert!(matches!(&runtime.directives[0], Directive::GoTo(s) if s.deref().eq("label")));
    }

    #[test]
    fn test_parsing_tree_sitter() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_crabscript::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse("#Testing", None).unwrap();
        println!("{tree:?}");
    }
}
