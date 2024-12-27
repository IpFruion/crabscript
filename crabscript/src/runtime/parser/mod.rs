pub mod error;
mod instruction;
mod tokenizer;

use std::ops::Deref;

use error::ParseErrorBuilder;
pub use error::{ParseError, ParseErrorKind};
pub use instruction::InstructionParsers;
use miette::SourceSpan;
use tokenizer::{tokenize, Cursor, Field, NodeKind};

use crate::context::Context;

use super::{directive::Directive, source::SourceCode, Runtime};

pub fn parse_runtime(ctx: Context, src: &SourceCode) -> Result<Runtime, ParseError> {
    let tree = tokenize(src).map_err(|builder| builder.build(src.clone()))?;
    let cursor = Cursor::new(src, tree.root_node());

    let mut directives = Vec::new();
    if cursor.is_node_kind(NodeKind::SourceFile) {
        if let Some(mut cursor) = cursor
            .named_child()
            .transpose()
            .map_err(|builder| builder.build(src.clone()))?
        {
            while cursor.node().is_named()
                || cursor
                    .goto_next_named_sibling()
                    .map_err(|builder| builder.build(src.clone()))?
            {
                parse_directive(&ctx, &mut cursor, &mut directives)
                    .map_err(|builder| builder.build(src.clone()))?;
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
    cursor: &mut Cursor<'_>,
    directives: &mut Vec<Directive>,
) -> Result<(), ParseErrorBuilder> {
    if !cursor.is_node_kind(NodeKind::Directive) {
        return Err(ParseErrorBuilder::new_tokenizer("not directive node").with_node(cursor.node()));
    }
    let cursor = cursor.named_child().transpose()?.ok_or_else(|| {
        ParseErrorBuilder::new_tokenizer("directive node has no child").with_node(cursor.node())
    })?;

    if cursor.is_node_kind(NodeKind::Instruction) {
        let span = cursor.span();
        let mut label = None;
        let mut output = None;
        if let Some(mut cursor) = cursor.opt_child_by_field(Field::Label)? {
            cursor.goto_next_sibling()?;
            label = Some(cursor.span());
        }
        if let Some(cursor) = cursor.opt_child_by_field(Field::Output)? {
            output = Some(cursor.span());
        }
        let label = label.map(Into::into);
        let output = output.map(Into::into);
        let instruction = ctx.get_parser().parse(&cursor)?;

        directives.push(Directive::Instruction {
            label,
            span: span.into(),
            instruction,
            output,
        });
        Ok(())
    } else if cursor.is_node_kind(NodeKind::Label) {
        let mut cursor = cursor.named_child().transpose()?.ok_or_else(|| {
            ParseErrorBuilder::new_tokenizer("label node has no child").with_node(cursor.node())
        })?;
        cursor.goto_next_named_sibling()?;
        directives.push(Directive::Label(cursor.span().into()));
        Ok(())
    } else {
        Err(ParseErrorBuilder::new_tokenizer(
            "missing directive type (either not handled or invalid)",
        )
        .with_node(cursor.node()))
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

    use crate::command::{Command, CommandError};

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
    fn test_parse_command_test_cmd_quotes() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let mut runtime = parse_runtime(context, &test_source_code(r#"tc "hello" 32"#)).unwrap();
        let instruction = match runtime.directives.pop().unwrap() {
            Directive::Instruction { instruction, .. } => instruction,
            _ => unreachable!(),
        };

        let test_cmd = instruction.as_command::<TestCommand>().unwrap();
        assert_eq!(&test_cmd.arg1, "\"hello\"");
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
            Directive::Instruction { label, output, .. } => {
                assert!(label.is_none());
                assert_eq!(output.as_deref(), Some("var"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_command_label() {
        let mut context = Context::default();
        context.add_command::<TestCommand>();

        let runtime = parse_runtime(context, &test_source_code(":label tc hello 32")).unwrap();
        assert_eq!(runtime.directives.len(), 1);
        match &runtime.directives[0] {
            Directive::Instruction { label, output, .. } => {
                assert!(output.is_none());
                assert_eq!(label.as_deref(), Some("label"));
            }
            _ => unreachable!(),
        }
    }
}
