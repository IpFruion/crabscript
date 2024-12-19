use tree_sitter::{Node, Tree, TreeCursor};

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
pub struct CrabscriptCursor<'a> {
    src: &'a SourceCode,
    cursor: TreeCursor<'a>,
}

impl<'a> CrabscriptCursor<'a> {
    pub fn new(src: &'a SourceCode, cursor: TreeCursor<'a>) -> Self {
        CrabscriptCursor { src, cursor }
    }
    pub fn goto_next_named_sibling(&mut self) -> Result<bool, ParseErrorBuilder> {
        let mut found = false;
        while self.cursor.goto_next_sibling() {
            if self.cursor.node().is_named() {
                found = true;
                break;
            }
        }
        self.check_node()?;
        Ok(found)
    }

    pub fn span(&self) -> Span<'a> {
        let node = self.cursor.node();
        Span {
            src: self.src,
            start: node.start_byte(),
            end: node.end_byte(),
        }
    }

    pub fn goto_next_sibling(&mut self) -> Result<bool, ParseErrorBuilder> {
        let res = self.cursor.goto_next_sibling();
        self.check_node()?;
        Ok(res)
    }

    pub fn goto_child(&mut self) -> Result<Option<Self>, ParseErrorBuilder> {
        Ok(if self.cursor.goto_first_child() {
            self.check_node()?;
            Some(CrabscriptCursor {
                src: self.src,
                cursor: self.cursor.clone(),
            })
        } else {
            None
        })
    }

    pub fn check_node(&self) -> Result<(), ParseErrorBuilder> {
        let node = self.cursor.node();
        if node.is_error() {
            return Err(ParseErrorBuilder::new_tokenizer(format!(
                "node {} error {}",
                node.grammar_name(),
                node.to_sexp()
            ))
            .with_node(node));
        }
        if node.is_missing() {
            return Err(ParseErrorBuilder::new_tokenizer(format!(
                "node {} missing {}",
                node.grammar_name(),
                node.to_sexp()
            ))
            .with_node(node));
        }
        Ok(())
    }

    #[inline]
    pub fn node(&self) -> Node<'a> {
        self.cursor.node()
    }

    /// Checks the current node is of a [NodeKind]
    #[inline]
    pub fn is_node_kind(&self, kind: NodeKind) -> bool {
        self.cursor.node().kind_id() == kind
    }

    /// Checks the current node if it is the field
    #[inline]
    pub fn is_field(&self, field: Field) -> bool {
        matches!(self.cursor.field_id(), Some(id) if id == field)
    }
}

impl<'a> std::fmt::Debug for CrabscriptCursor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Add source debug information?
        f.debug_tuple("CrabscriptCursor")
            .field(&self.cursor.node().grammar_name())
            .field(&self.cursor.node())
            .field(&self.cursor.node().to_sexp())
            .finish()
    }
}

impl<'a> Drop for CrabscriptCursor<'a> {
    fn drop(&mut self) {
        self.cursor.goto_parent();
    }
}
