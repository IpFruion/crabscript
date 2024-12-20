/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "crabscript",
  rules: {
    source_file: ($) => choice(repeat(choice($._line, /[\n\r]+/)), $._eoi),
    _line: ($) => seq(choice($._comment, $.directive), $._eoi),
    directive: ($) => choice($.instruction, $.goto),
    instruction: ($) =>
      seq(
        repeat($._whitespace),
        field("label", optional($._instruction_label)),
        field("output", optional($._output)),
        field("cmd", $.ident),
        optional($._args),
      ),
    goto: ($) => $._label,
    _args: ($) => repeat1(seq(repeat1($._whitespace), field("arg", $.arg))),
    arg: (_) => choice(/"(?:[^"\\]|\\.)*"/, repeat1(/[^ \t\n]/)),
    _output: ($) =>
      seq($.ident, repeat($._whitespace), "=", repeat($._whitespace)),
    _label: ($) => seq(":", $.ident),
    ident: (_) => /[a-zA-Z0-9_]+/,
    _instruction_label: ($) => seq($._label, repeat1($._whitespace)),
    _comment: ($) => seq(repeat($._whitespace), "#", /[^\n]*/),
    _whitespace: (_) => /[ \t]/,
    _eoi: ($) => prec(2, choice(/[\n\r]*/, $._eof)),
    _eof: (_) => "\0",
  },
  extras: (_) => [],
});
