use tree_sitter::{Node, Tree};

use crate::runtime::source::SourceCode;

use super::{error::ParseErrorBuilder, Span};

include!(concat!(env!("OUT_DIR"), "/node_kind.rs"));

const TIMEOUT_MICROS: u64 = 500_000; // half a second

/// Produces the syntax tree for crabscript
pub fn tokenize<S: AsRef<str>>(src: S) -> Result<Tree, ParseErrorBuilder> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_crabscript::LANGUAGE.into())
        .expect("versions to match since this project is coupled with tree_sitter_crabscript");
    parser.set_timeout_micros(TIMEOUT_MICROS);
    parser
        .parse(src.as_ref(), None)
        .ok_or_else(|| ParseErrorBuilder::new_tokenizer("parse tree not found or timed out"))
}

/// A wrapper to the [tree_sitter::TreeCursor] that includes being able to back to parent
pub struct Cursor<'a> {
    src: &'a SourceCode,
    current: Node<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a SourceCode, root: Node<'a>) -> Self {
        Cursor { src, current: root }
    }

    pub fn goto_next_named_sibling(&mut self) -> Result<bool, ParseErrorBuilder> {
        self.current
            .next_named_sibling()
            .map(|node| {
                self.current = node;
                self.check_node()?;
                Ok(true)
            })
            .unwrap_or(Ok(false))
    }

    pub fn span(&self) -> Span<'a> {
        Span {
            src: self.src,
            start: self.current.start_byte(),
            end: self.current.end_byte(),
        }
    }

    pub fn goto_next_sibling(&mut self) -> Result<bool, ParseErrorBuilder> {
        self.current
            .next_sibling()
            .map(|node| {
                self.current = node;
                self.check_node()?;
                Ok(true)
            })
            .unwrap_or(Ok(false))
    }

    pub fn named_child(&self) -> Option<Result<Self, ParseErrorBuilder>> {
        self.current.named_child(0).map(|node| {
            let cursor = Cursor {
                src: self.src,
                current: node,
            };
            cursor.check_node()?;
            Ok(cursor)
        })
    }

    pub fn check_node(&self) -> Result<(), ParseErrorBuilder> {
        if self.current.is_error() {
            return Err(ParseErrorBuilder::new_tokenizer(format!(
                "node {} error {}",
                self.current.grammar_name(),
                self.current.to_sexp()
            ))
            .with_node(self.current));
        }
        if self.current.is_missing() {
            return Err(ParseErrorBuilder::new_tokenizer(format!(
                "node {} missing {}",
                self.current.grammar_name(),
                self.current.to_sexp()
            ))
            .with_node(self.current));
        }
        Ok(())
    }

    pub fn opt_child_by_field(&self, field: Field) -> Result<Option<Self>, ParseErrorBuilder> {
        self.current
            .child_by_field_id(field as u16)
            .map(|node| {
                let cursor = Cursor {
                    src: self.src,
                    current: node,
                };
                cursor.check_node()?;
                Ok(cursor)
            })
            .transpose()
    }

    pub fn child_by_field(&self, field: Field) -> Result<Self, ParseErrorBuilder> {
        self.current
            .child_by_field_id(field as u16)
            .ok_or_else(|| {
                ParseErrorBuilder::new_tokenizer(format!(
                    "{} has no child with field {:?}",
                    self.current.grammar_name(),
                    field
                ))
                .with_node(self.current)
            })
            .and_then(|node| {
                let cursor = Cursor {
                    src: self.src,
                    current: node,
                };
                cursor.check_node()?;
                Ok(cursor)
            })
    }

    pub fn children_by_field(&self, field: Field) -> Result<Vec<Self>, ParseErrorBuilder> {
        let mut cursor = self.current.walk();
        self.current
            .children_by_field_id(field.id(), &mut cursor)
            .map(|node| self.new_child(node))
            .collect()
    }

    fn new_child(&self, node: Node<'a>) -> Result<Self, ParseErrorBuilder> {
        let cursor = Cursor {
            src: self.src,
            current: node,
        };
        cursor.check_node()?;
        Ok(cursor)
    }

    #[inline]
    pub fn node(&self) -> Node<'a> {
        self.current
    }

    /// Checks the current node is of a [NodeKind]
    #[inline]
    pub fn is_node_kind(&self, kind: NodeKind) -> bool {
        self.current.kind_id() == kind
    }
}

impl<'a> std::fmt::Debug for Cursor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Add source debug information?
        f.debug_tuple("CrabscriptCursor")
            .field(&self.current.grammar_name())
            .field(&self.current)
            .field(&self.current.to_sexp())
            .finish()
    }
}
