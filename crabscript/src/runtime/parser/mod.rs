pub mod error;
mod tokenizer;
use std::{collections::HashMap, ops::Deref, rc::Rc};

use clap::{CommandFactory, FromArgMatches};
use error::ParseErrorBuilder;
pub use error::{ParseError, ParseErrorKind};
use miette::SourceSpan;
use tokenizer::{tokenize, CrabscriptCursor, Field, NodeKind};

use crate::{
    command::{Command, CommandOutput},
    context::Context,
};

use super::{source::SourceCode, Directive, Instruction, Runtime};

pub fn parse_runtime(ctx: Context, src: &SourceCode) -> Result<Runtime, ParseError> {
    let tree = tokenize(src.deref()).map_err(|builder| builder.build(src.clone()))?;
    let mut cursor = CrabscriptCursor::new(src, tree.walk());

    let mut directives = Vec::new();
    if cursor.is_node_kind(NodeKind::SourceFile) {
        if let Some(mut cursor) = cursor
            .goto_child()
            .map_err(|builder| builder.build(src.clone()))?
        {
            loop {
                let directive = parse_directive(&ctx, &mut cursor)
                    .map_err(|builder| builder.build(src.clone()))?;
                directives.push(directive);
                if !cursor
                    .goto_next_named_sibling()
                    .map_err(|builder| builder.build(src.clone()))?
                {
                    break;
                }
            }
        }
    }

    Ok(Runtime::new(ctx, directives))
}

fn parse_directive(
    ctx: &Context,
    cursor: &mut CrabscriptCursor<'_>,
) -> Result<Directive, ParseErrorBuilder> {
    if !cursor.is_node_kind(NodeKind::Directive) {
        return Err(ParseErrorBuilder::new_tokenizer("not directive node").with_node(cursor.node()));
    }
    let mut cursor = cursor.goto_child()?.ok_or_else(|| {
        ParseErrorBuilder::new_tokenizer("directive node has no child").with_node(cursor.node())
    })?;

    if cursor.node().kind_id() == NodeKind::Instruction {
        let mut cursor = cursor.goto_child()?.ok_or_else(|| {
            ParseErrorBuilder::new_tokenizer("instruction node has no child")
                .with_node(cursor.node())
        })?;
        let span = cursor.span();
        let mut label = None;
        let mut output = None;
        if cursor.is_field(Field::Label) {
            cursor.goto_next_named_sibling()?;
            label = Some(cursor.span());
            cursor.goto_next_named_sibling()?;
        }
        if cursor.is_field(Field::Output) {
            output = Some(cursor.span());
            cursor.goto_next_named_sibling()?;
        }
        if !cursor.is_field(Field::Cmd) {
            return Err(
                ParseErrorBuilder::new_tokenizer("cmd field not found on instruction")
                    .with_node(cursor.node()),
            );
        }
        let name = cursor.span();
        let parser = ctx.get_parser(&name).ok_or_else(|| {
            ParseErrorBuilder::new(ParseErrorKind::CommandNotFound)
                .with_span(name.into())
                .with_reason("command not found")
        })?;
        let mut args = Vec::new();
        while cursor.goto_next_sibling()? && cursor.is_field(Field::Arg) {
            args.push(cursor.span());
        }

        let instruction = parser.0(name, &args)?;

        Ok(Directive::Instruction {
            label: label.map(Into::into),
            span: span.into(),
            instruction,
            output: output.map(Into::into),
        })
    } else if cursor.is_node_kind(NodeKind::Goto) {
        let mut cursor = cursor.goto_child()?.ok_or_else(|| {
            ParseErrorBuilder::new_tokenizer("goto node has no child").with_node(cursor.node())
        })?;
        cursor.goto_next_named_sibling()?;
        Ok(Directive::GoTo(cursor.span().into()))
    } else {
        Err(ParseErrorBuilder::new_tokenizer(
            "missing directive type (either not handled or invalid)",
        )
        .with_node(cursor.node()))
    }
}

#[derive(Clone)]
pub struct InstructionParser(Rc<InstructionParserFn>);

type InstructionParserFn = dyn Fn(Span<'_>, &[Span<'_>]) -> Result<Instruction, ParseErrorBuilder>;

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
                    .try_get_matches_from(args.iter().map(|span| span.deref()))
                    .map_err(|err| ParseErrorBuilder::from_clap_error(err, cmd_span, args))?;
                let t = T::from_arg_matches(&matches)
                    .map_err(|err| ParseErrorBuilder::from_clap_error(err, cmd_span, args))?;
                Ok(Instruction::new_generic(t))
            }))
        });
        //TODO: Handle aliases
    }
}

#[derive(Clone, Copy)]
pub struct Span<'a> {
    src: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Deref for Span<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.src[self.start..self.end]
    }
}
impl<'a> From<&Span<'a>> for SourceSpan {
    fn from(val: &Span<'a>) -> Self {
        (*val).into()
    }
}

impl<'a> From<Span<'a>> for SourceSpan {
    fn from(val: Span<'a>) -> Self {
        SourceSpan::new(val.start.into(), val.end - val.start)
    }
}

impl<'a> From<Span<'a>> for Box<str> {
    fn from(value: Span<'a>) -> Self {
        value.deref().into()
    }
}

#[cfg(test)]
mod tests {

    use std::ops::Deref;

    use crate::command::CommandError;

    use super::*;

    fn test_source_code(value: &str) -> SourceCode {
        SourceCode::named("parse_test", value)
    }

    #[test]
    fn test_parse_empty() {
        parse_runtime(Default::default(), &test_source_code("")).unwrap();
    }

    #[test]
    fn test_parse_command_not_found() {
        let err =
            parse_runtime(Default::default(), &test_source_code("mkdir testing")).unwrap_err();

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

        let err = parse_runtime(context, &test_source_code("tc")).unwrap_err();

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

        let err = parse_runtime(context, &test_source_code("tc hello world")).unwrap_err();

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

        let runtime = parse_runtime(context, &test_source_code("tc hello 32")).unwrap();

        assert_eq!(runtime.directives.len(), 1);
    }

    #[test]
    fn test_parse_command_test_cmd_invalid_opt_arg() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let err = parse_runtime(context, &test_source_code("tc hello 32 --arg3")).unwrap_err();

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

        let runtime = parse_runtime(context, &test_source_code("var = tc hello 32")).unwrap();
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

        let runtime = parse_runtime(context, &test_source_code(":label tc hello 32")).unwrap();
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

        let runtime = parse_runtime(context, &test_source_code(":label")).unwrap();
        assert_eq!(runtime.directives.len(), 1);
        assert!(matches!(&runtime.directives[0], Directive::GoTo(s) if s.deref().eq("label")));
    }
}
