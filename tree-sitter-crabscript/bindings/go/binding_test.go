package tree_sitter_crabscript_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_crabscript "github.com/tree-sitter/tree-sitter-crabscript/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_crabscript.Language())
	if language == nil {
		t.Errorf("Error loading Crabscript grammar")
	}
}
