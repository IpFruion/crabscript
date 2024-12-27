/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "crabscript",
  rules: {
    source_file: ($) => choice(repeat($._line), $._eoi),
    _line: ($) =>
      seq(
        repeat($._whitespace),
        choice(seq(choice($._comment, $.directive), $._eoi), /[\n\r]+/),
      ),
    directive: ($) => choice($.instruction, $.label, $.cond_if, $.cond_while),
    instruction: ($) =>
      seq(
        field("label", optional($._instruction_label)),
        field("output", optional($._output)),
        field("cmd", $.ident),
        repeat($._arg_entry),
        repeat($._whitespace),
      ),
    label: ($) => seq($._label, repeat($._whitespace)),

    // Conditionals
    cond_if: ($) => seq("if", $._whitespace, $._condition, $._cond_body),
    cond_while: ($) => seq("while", $._whitespace, $._condition, $._cond_body),
    cond_for: ($) =>
      seq("for", $._whitespace, $.ident, "in", $._condition, $._cond_body),
    _condition: ($) =>
      choice(seq($.ident, repeat($._arg_entry)), $.variable_arg),
    _cond_body: ($) =>
      seq(
        /[\n\r]/,
        repeat($._line),
        repeat($._whitespace),
        "end",
        repeat($._whitespace),
      ),
    _arg_entry: ($) => seq(repeat1($._whitespace), field("arg", $.arg)),
    arg: ($) => choice($.variable_arg, $._quoted_arg, $._normal_arg),
    variable_arg: ($) => seq("${", $.ident, "}"),
    _quoted_arg: (_) => seq('"', /(?:[^"\\]|\\.)*/, '"'),
    _normal_arg: (_) => repeat1(/[^ \t\n]/),
    _output: ($) =>
      seq($.ident, repeat($._whitespace), "=", repeat($._whitespace)),
    _label: ($) => seq(":", $.ident),
    ident: (_) => /[a-zA-Z0-9_]+/,
    _instruction_label: ($) => seq($._label, repeat1($._whitespace)),
    _comment: (_) => seq("#", /[^\n]*/),
    _whitespace: (_) => /[ \t]/,
    _eoi: ($) => prec(2, choice(/[\n\r]*/, $._eof)),
    _eof: (_) => "\0",
  },
  extras: (_) => [],
});
