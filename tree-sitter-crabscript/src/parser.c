#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 55
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 28
#define ALIAS_COUNT 0
#define TOKEN_COUNT 12
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 19

enum ts_symbol_identifiers {
  aux_sym_source_file_token1 = 1,
  aux_sym_arg_token1 = 2,
  aux_sym_arg_token2 = 3,
  anon_sym_EQ = 4,
  anon_sym_COLON = 5,
  sym_ident = 6,
  anon_sym_POUND = 7,
  aux_sym__comment_token1 = 8,
  sym__whitespace = 9,
  aux_sym__eoi_token1 = 10,
  anon_sym_NULL = 11,
  sym_source_file = 12,
  sym__line = 13,
  sym_directive = 14,
  sym_instruction = 15,
  sym_goto = 16,
  aux_sym__args = 17,
  sym_arg = 18,
  sym__output = 19,
  sym__label = 20,
  sym__instruction_label = 21,
  sym__comment = 22,
  sym__eoi = 23,
  sym__eof = 24,
  aux_sym_source_file_repeat1 = 25,
  aux_sym_instruction_repeat1 = 26,
  aux_sym_arg_repeat1 = 27,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_source_file_token1] = "source_file_token1",
  [aux_sym_arg_token1] = "arg_token1",
  [aux_sym_arg_token2] = "arg_token2",
  [anon_sym_EQ] = "=",
  [anon_sym_COLON] = ":",
  [sym_ident] = "ident",
  [anon_sym_POUND] = "#",
  [aux_sym__comment_token1] = "_comment_token1",
  [sym__whitespace] = "_whitespace",
  [aux_sym__eoi_token1] = "_eoi_token1",
  [anon_sym_NULL] = "\0",
  [sym_source_file] = "source_file",
  [sym__line] = "_line",
  [sym_directive] = "directive",
  [sym_instruction] = "instruction",
  [sym_goto] = "goto",
  [aux_sym__args] = "_args",
  [sym_arg] = "arg",
  [sym__output] = "_output",
  [sym__label] = "_label",
  [sym__instruction_label] = "_instruction_label",
  [sym__comment] = "_comment",
  [sym__eoi] = "_eoi",
  [sym__eof] = "_eof",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_instruction_repeat1] = "instruction_repeat1",
  [aux_sym_arg_repeat1] = "arg_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_source_file_token1] = aux_sym_source_file_token1,
  [aux_sym_arg_token1] = aux_sym_arg_token1,
  [aux_sym_arg_token2] = aux_sym_arg_token2,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_ident] = sym_ident,
  [anon_sym_POUND] = anon_sym_POUND,
  [aux_sym__comment_token1] = aux_sym__comment_token1,
  [sym__whitespace] = sym__whitespace,
  [aux_sym__eoi_token1] = aux_sym__eoi_token1,
  [anon_sym_NULL] = anon_sym_NULL,
  [sym_source_file] = sym_source_file,
  [sym__line] = sym__line,
  [sym_directive] = sym_directive,
  [sym_instruction] = sym_instruction,
  [sym_goto] = sym_goto,
  [aux_sym__args] = aux_sym__args,
  [sym_arg] = sym_arg,
  [sym__output] = sym__output,
  [sym__label] = sym__label,
  [sym__instruction_label] = sym__instruction_label,
  [sym__comment] = sym__comment,
  [sym__eoi] = sym__eoi,
  [sym__eof] = sym__eof,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_instruction_repeat1] = aux_sym_instruction_repeat1,
  [aux_sym_arg_repeat1] = aux_sym_arg_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arg_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arg_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__comment_token1] = {
    .visible = false,
    .named = false,
  },
  [sym__whitespace] = {
    .visible = false,
    .named = true,
  },
  [aux_sym__eoi_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_NULL] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__line] = {
    .visible = false,
    .named = true,
  },
  [sym_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_instruction] = {
    .visible = true,
    .named = true,
  },
  [sym_goto] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__args] = {
    .visible = false,
    .named = false,
  },
  [sym_arg] = {
    .visible = true,
    .named = true,
  },
  [sym__output] = {
    .visible = false,
    .named = true,
  },
  [sym__label] = {
    .visible = false,
    .named = true,
  },
  [sym__instruction_label] = {
    .visible = false,
    .named = true,
  },
  [sym__comment] = {
    .visible = false,
    .named = true,
  },
  [sym__eoi] = {
    .visible = false,
    .named = true,
  },
  [sym__eof] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_instruction_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arg_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_arg = 1,
  field_cmd = 2,
  field_label = 3,
  field_output = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arg] = "arg",
  [field_cmd] = "cmd",
  [field_label] = "label",
  [field_output] = "output",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 2},
  [5] = {.index = 7, .length = 1},
  [6] = {.index = 8, .length = 2},
  [7] = {.index = 10, .length = 1},
  [8] = {.index = 11, .length = 3},
  [9] = {.index = 14, .length = 3},
  [10] = {.index = 17, .length = 3},
  [11] = {.index = 20, .length = 2},
  [12] = {.index = 22, .length = 2},
  [13] = {.index = 24, .length = 2},
  [14] = {.index = 26, .length = 4},
  [15] = {.index = 30, .length = 3},
  [16] = {.index = 33, .length = 3},
  [17] = {.index = 36, .length = 3},
  [18] = {.index = 39, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_cmd, 0},
  [1] =
    {field_arg, 1, .inherited = true},
    {field_cmd, 0},
  [3] =
    {field_cmd, 1},
    {field_output, 0},
  [5] =
    {field_cmd, 1},
    {field_label, 0},
  [7] =
    {field_cmd, 1},
  [8] =
    {field_arg, 0, .inherited = true},
    {field_arg, 1, .inherited = true},
  [10] =
    {field_arg, 1},
  [11] =
    {field_arg, 2, .inherited = true},
    {field_cmd, 1},
    {field_output, 0},
  [14] =
    {field_arg, 2, .inherited = true},
    {field_cmd, 1},
    {field_label, 0},
  [17] =
    {field_cmd, 2},
    {field_label, 0},
    {field_output, 1},
  [20] =
    {field_arg, 2, .inherited = true},
    {field_cmd, 1},
  [22] =
    {field_cmd, 2},
    {field_output, 1},
  [24] =
    {field_cmd, 2},
    {field_label, 1},
  [26] =
    {field_arg, 3, .inherited = true},
    {field_cmd, 2},
    {field_label, 0},
    {field_output, 1},
  [30] =
    {field_arg, 3, .inherited = true},
    {field_cmd, 2},
    {field_output, 1},
  [33] =
    {field_arg, 3, .inherited = true},
    {field_cmd, 2},
    {field_label, 1},
  [36] =
    {field_cmd, 3},
    {field_label, 1},
    {field_output, 2},
  [39] =
    {field_arg, 4, .inherited = true},
    {field_cmd, 3},
    {field_label, 1},
    {field_output, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 12,
  [29] = 29,
  [30] = 30,
  [31] = 12,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if (eof) ADVANCE(5);
      ADVANCE_MAP(
        0, 22,
        '\n', 20,
        '\r', 10,
        '"', 9,
        '#', 14,
        ':', 12,
        '=', 11,
        '\t', 16,
        ' ', 16,
      );
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(9);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n') ADVANCE(8);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n') ADVANCE(8);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(7);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(aux_sym_source_file_token1);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(aux_sym_arg_token1);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(aux_sym_arg_token2);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(aux_sym_arg_token2);
      if (lookahead == '"') ADVANCE(7);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(aux_sym_arg_token2);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(20);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_ident);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(aux_sym__comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym__whitespace);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if ((!eof && lookahead == 00)) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(20);
      if (lookahead == '\r') ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if ((!eof && lookahead == 00)) ADVANCE(22);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(20);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if ((!eof && lookahead == 00)) ADVANCE(22);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(aux_sym__eoi_token1);
      if (eof) ADVANCE(5);
      if ((!eof && lookahead == 00)) ADVANCE(22);
      if (lookahead == '#') ADVANCE(14);
      if (lookahead == ':') ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_NULL);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 21},
  [2] = {.lex_state = 21},
  [3] = {.lex_state = 21},
  [4] = {.lex_state = 21},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 21},
  [7] = {.lex_state = 18},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 18},
  [10] = {.lex_state = 18},
  [11] = {.lex_state = 18},
  [12] = {.lex_state = 21},
  [13] = {.lex_state = 18},
  [14] = {.lex_state = 18},
  [15] = {.lex_state = 18},
  [16] = {.lex_state = 17},
  [17] = {.lex_state = 18},
  [18] = {.lex_state = 18},
  [19] = {.lex_state = 18},
  [20] = {.lex_state = 18},
  [21] = {.lex_state = 18},
  [22] = {.lex_state = 18},
  [23] = {.lex_state = 17},
  [24] = {.lex_state = 18},
  [25] = {.lex_state = 18},
  [26] = {.lex_state = 18},
  [27] = {.lex_state = 18},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 18},
  [30] = {.lex_state = 19},
  [31] = {.lex_state = 2},
  [32] = {.lex_state = 21},
  [33] = {.lex_state = 18},
  [34] = {.lex_state = 18},
  [35] = {.lex_state = 21},
  [36] = {.lex_state = 21},
  [37] = {.lex_state = 21},
  [38] = {.lex_state = 21},
  [39] = {.lex_state = 18},
  [40] = {.lex_state = 19},
  [41] = {.lex_state = 21},
  [42] = {.lex_state = 19},
  [43] = {.lex_state = 19},
  [44] = {.lex_state = 21},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 21},
  [47] = {.lex_state = 15},
  [48] = {.lex_state = 21},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 21},
  [51] = {.lex_state = 21},
  [52] = {.lex_state = 21},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 15},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_arg_token1] = ACTIONS(1),
    [aux_sym_arg_token2] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [sym__whitespace] = ACTIONS(1),
    [aux_sym__eoi_token1] = ACTIONS(1),
    [anon_sym_NULL] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(49),
    [sym__line] = STATE(3),
    [sym_directive] = STATE(30),
    [sym_instruction] = STATE(43),
    [sym_goto] = STATE(43),
    [sym__output] = STATE(50),
    [sym__label] = STATE(29),
    [sym__instruction_label] = STATE(44),
    [sym__comment] = STATE(30),
    [sym__eoi] = STATE(53),
    [sym__eof] = STATE(53),
    [aux_sym_source_file_repeat1] = STATE(3),
    [aux_sym_instruction_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_source_file_token1] = ACTIONS(5),
    [anon_sym_COLON] = ACTIONS(7),
    [sym_ident] = ACTIONS(9),
    [anon_sym_POUND] = ACTIONS(11),
    [sym__whitespace] = ACTIONS(13),
    [aux_sym__eoi_token1] = ACTIONS(15),
    [anon_sym_NULL] = ACTIONS(17),
  },
  [2] = {
    [sym__line] = STATE(2),
    [sym_directive] = STATE(30),
    [sym_instruction] = STATE(43),
    [sym_goto] = STATE(43),
    [sym__output] = STATE(50),
    [sym__label] = STATE(29),
    [sym__instruction_label] = STATE(44),
    [sym__comment] = STATE(30),
    [aux_sym_source_file_repeat1] = STATE(2),
    [aux_sym_instruction_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(19),
    [aux_sym_source_file_token1] = ACTIONS(21),
    [anon_sym_COLON] = ACTIONS(24),
    [sym_ident] = ACTIONS(27),
    [anon_sym_POUND] = ACTIONS(30),
    [sym__whitespace] = ACTIONS(33),
  },
  [3] = {
    [sym__line] = STATE(2),
    [sym_directive] = STATE(30),
    [sym_instruction] = STATE(43),
    [sym_goto] = STATE(43),
    [sym__output] = STATE(50),
    [sym__label] = STATE(29),
    [sym__instruction_label] = STATE(44),
    [sym__comment] = STATE(30),
    [aux_sym_source_file_repeat1] = STATE(2),
    [aux_sym_instruction_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(36),
    [aux_sym_source_file_token1] = ACTIONS(38),
    [anon_sym_COLON] = ACTIONS(7),
    [sym_ident] = ACTIONS(9),
    [anon_sym_POUND] = ACTIONS(11),
    [sym__whitespace] = ACTIONS(13),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(7), 1,
      anon_sym_COLON,
    ACTIONS(40), 1,
      sym_ident,
    ACTIONS(42), 1,
      anon_sym_POUND,
    ACTIONS(44), 1,
      sym__whitespace,
    STATE(12), 1,
      aux_sym_instruction_repeat1,
    STATE(41), 1,
      sym__instruction_label,
    STATE(45), 1,
      sym__label,
    STATE(48), 1,
      sym__output,
  [25] = 7,
    ACTIONS(46), 1,
      aux_sym_arg_token1,
    ACTIONS(48), 1,
      aux_sym_arg_token2,
    ACTIONS(50), 1,
      anon_sym_EQ,
    ACTIONS(52), 1,
      sym__whitespace,
    STATE(16), 1,
      aux_sym_arg_repeat1,
    STATE(28), 1,
      aux_sym_instruction_repeat1,
    STATE(39), 1,
      sym_arg,
  [47] = 1,
    ACTIONS(54), 6,
      ts_builtin_sym_end,
      aux_sym_source_file_token1,
      anon_sym_COLON,
      sym_ident,
      anon_sym_POUND,
      sym__whitespace,
  [56] = 6,
    ACTIONS(56), 1,
      anon_sym_EQ,
    ACTIONS(58), 1,
      sym__whitespace,
    ACTIONS(60), 1,
      aux_sym__eoi_token1,
    ACTIONS(62), 1,
      anon_sym_NULL,
    STATE(5), 1,
      aux_sym_instruction_repeat1,
    STATE(18), 1,
      aux_sym__args,
  [75] = 6,
    ACTIONS(46), 1,
      aux_sym_arg_token1,
    ACTIONS(48), 1,
      aux_sym_arg_token2,
    ACTIONS(64), 1,
      sym__whitespace,
    STATE(16), 1,
      aux_sym_arg_repeat1,
    STATE(31), 1,
      aux_sym_instruction_repeat1,
    STATE(39), 1,
      sym_arg,
  [94] = 6,
    ACTIONS(56), 1,
      anon_sym_EQ,
    ACTIONS(58), 1,
      sym__whitespace,
    ACTIONS(66), 1,
      aux_sym__eoi_token1,
    ACTIONS(68), 1,
      anon_sym_NULL,
    STATE(5), 1,
      aux_sym_instruction_repeat1,
    STATE(21), 1,
      aux_sym__args,
  [113] = 6,
    ACTIONS(56), 1,
      anon_sym_EQ,
    ACTIONS(58), 1,
      sym__whitespace,
    ACTIONS(70), 1,
      aux_sym__eoi_token1,
    ACTIONS(72), 1,
      anon_sym_NULL,
    STATE(5), 1,
      aux_sym_instruction_repeat1,
    STATE(26), 1,
      aux_sym__args,
  [132] = 6,
    ACTIONS(56), 1,
      anon_sym_EQ,
    ACTIONS(58), 1,
      sym__whitespace,
    ACTIONS(74), 1,
      aux_sym__eoi_token1,
    ACTIONS(76), 1,
      anon_sym_NULL,
    STATE(5), 1,
      aux_sym_instruction_repeat1,
    STATE(13), 1,
      aux_sym__args,
  [151] = 3,
    ACTIONS(80), 1,
      sym__whitespace,
    STATE(12), 1,
      aux_sym_instruction_repeat1,
    ACTIONS(78), 3,
      anon_sym_COLON,
      sym_ident,
      anon_sym_POUND,
  [163] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(85), 1,
      aux_sym__eoi_token1,
    ACTIONS(87), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [179] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(89), 1,
      aux_sym__eoi_token1,
    ACTIONS(91), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(20), 1,
      aux_sym__args,
  [195] = 5,
    ACTIONS(93), 1,
      sym__whitespace,
    ACTIONS(96), 1,
      aux_sym__eoi_token1,
    ACTIONS(98), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [211] = 4,
    ACTIONS(100), 1,
      aux_sym_arg_token2,
    ACTIONS(104), 1,
      aux_sym__eoi_token1,
    STATE(23), 1,
      aux_sym_arg_repeat1,
    ACTIONS(102), 2,
      sym__whitespace,
      anon_sym_NULL,
  [225] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(106), 1,
      aux_sym__eoi_token1,
    ACTIONS(108), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(22), 1,
      aux_sym__args,
  [241] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(110), 1,
      aux_sym__eoi_token1,
    ACTIONS(112), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [257] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(114), 1,
      aux_sym__eoi_token1,
    ACTIONS(116), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(24), 1,
      aux_sym__args,
  [273] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(118), 1,
      aux_sym__eoi_token1,
    ACTIONS(120), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [289] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(122), 1,
      aux_sym__eoi_token1,
    ACTIONS(124), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [305] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(126), 1,
      aux_sym__eoi_token1,
    ACTIONS(128), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [321] = 4,
    ACTIONS(130), 1,
      aux_sym_arg_token2,
    ACTIONS(135), 1,
      aux_sym__eoi_token1,
    STATE(23), 1,
      aux_sym_arg_repeat1,
    ACTIONS(133), 2,
      sym__whitespace,
      anon_sym_NULL,
  [335] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(137), 1,
      aux_sym__eoi_token1,
    ACTIONS(139), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [351] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(141), 1,
      aux_sym__eoi_token1,
    ACTIONS(143), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(27), 1,
      aux_sym__args,
  [367] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(145), 1,
      aux_sym__eoi_token1,
    ACTIONS(147), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [383] = 5,
    ACTIONS(83), 1,
      sym__whitespace,
    ACTIONS(149), 1,
      aux_sym__eoi_token1,
    ACTIONS(151), 1,
      anon_sym_NULL,
    STATE(8), 1,
      aux_sym_instruction_repeat1,
    STATE(15), 1,
      aux_sym__args,
  [399] = 4,
    ACTIONS(153), 1,
      aux_sym_arg_token2,
    ACTIONS(155), 1,
      sym__whitespace,
    STATE(28), 1,
      aux_sym_instruction_repeat1,
    ACTIONS(78), 2,
      aux_sym_arg_token1,
      anon_sym_EQ,
  [413] = 4,
    ACTIONS(158), 1,
      sym__whitespace,
    ACTIONS(160), 1,
      aux_sym__eoi_token1,
    ACTIONS(162), 1,
      anon_sym_NULL,
    STATE(35), 1,
      aux_sym_instruction_repeat1,
  [426] = 3,
    ACTIONS(164), 1,
      aux_sym__eoi_token1,
    ACTIONS(166), 1,
      anon_sym_NULL,
    STATE(6), 2,
      sym__eoi,
      sym__eof,
  [437] = 4,
    ACTIONS(78), 1,
      aux_sym_arg_token1,
    ACTIONS(153), 1,
      aux_sym_arg_token2,
    ACTIONS(168), 1,
      sym__whitespace,
    STATE(31), 1,
      aux_sym_instruction_repeat1,
  [450] = 3,
    ACTIONS(44), 1,
      sym__whitespace,
    ACTIONS(171), 1,
      sym_ident,
    STATE(12), 1,
      aux_sym_instruction_repeat1,
  [460] = 2,
    ACTIONS(175), 1,
      aux_sym__eoi_token1,
    ACTIONS(173), 2,
      sym__whitespace,
      anon_sym_NULL,
  [468] = 2,
    ACTIONS(104), 1,
      aux_sym__eoi_token1,
    ACTIONS(102), 2,
      sym__whitespace,
      anon_sym_NULL,
  [476] = 3,
    ACTIONS(44), 1,
      sym__whitespace,
    ACTIONS(177), 1,
      sym_ident,
    STATE(12), 1,
      aux_sym_instruction_repeat1,
  [486] = 3,
    ACTIONS(44), 1,
      sym__whitespace,
    ACTIONS(179), 1,
      sym_ident,
    STATE(12), 1,
      aux_sym_instruction_repeat1,
  [496] = 3,
    ACTIONS(179), 1,
      sym_ident,
    ACTIONS(181), 1,
      sym__whitespace,
    STATE(32), 1,
      aux_sym_instruction_repeat1,
  [506] = 3,
    ACTIONS(183), 1,
      sym_ident,
    ACTIONS(185), 1,
      sym__whitespace,
    STATE(36), 1,
      aux_sym_instruction_repeat1,
  [516] = 2,
    ACTIONS(189), 1,
      aux_sym__eoi_token1,
    ACTIONS(187), 2,
      sym__whitespace,
      anon_sym_NULL,
  [524] = 2,
    ACTIONS(191), 1,
      aux_sym__eoi_token1,
    ACTIONS(193), 1,
      anon_sym_NULL,
  [531] = 2,
    ACTIONS(195), 1,
      sym_ident,
    STATE(46), 1,
      sym__output,
  [538] = 2,
    ACTIONS(197), 1,
      aux_sym__eoi_token1,
    ACTIONS(199), 1,
      anon_sym_NULL,
  [545] = 2,
    ACTIONS(201), 1,
      aux_sym__eoi_token1,
    ACTIONS(203), 1,
      anon_sym_NULL,
  [552] = 2,
    ACTIONS(205), 1,
      sym_ident,
    STATE(51), 1,
      sym__output,
  [559] = 2,
    ACTIONS(158), 1,
      sym__whitespace,
    STATE(35), 1,
      aux_sym_instruction_repeat1,
  [566] = 1,
    ACTIONS(207), 1,
      sym_ident,
  [570] = 1,
    ACTIONS(209), 1,
      aux_sym__comment_token1,
  [574] = 1,
    ACTIONS(211), 1,
      sym_ident,
  [578] = 1,
    ACTIONS(213), 1,
      ts_builtin_sym_end,
  [582] = 1,
    ACTIONS(215), 1,
      sym_ident,
  [586] = 1,
    ACTIONS(217), 1,
      sym_ident,
  [590] = 1,
    ACTIONS(219), 1,
      sym_ident,
  [594] = 1,
    ACTIONS(36), 1,
      ts_builtin_sym_end,
  [598] = 1,
    ACTIONS(221), 1,
      aux_sym__comment_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 25,
  [SMALL_STATE(6)] = 47,
  [SMALL_STATE(7)] = 56,
  [SMALL_STATE(8)] = 75,
  [SMALL_STATE(9)] = 94,
  [SMALL_STATE(10)] = 113,
  [SMALL_STATE(11)] = 132,
  [SMALL_STATE(12)] = 151,
  [SMALL_STATE(13)] = 163,
  [SMALL_STATE(14)] = 179,
  [SMALL_STATE(15)] = 195,
  [SMALL_STATE(16)] = 211,
  [SMALL_STATE(17)] = 225,
  [SMALL_STATE(18)] = 241,
  [SMALL_STATE(19)] = 257,
  [SMALL_STATE(20)] = 273,
  [SMALL_STATE(21)] = 289,
  [SMALL_STATE(22)] = 305,
  [SMALL_STATE(23)] = 321,
  [SMALL_STATE(24)] = 335,
  [SMALL_STATE(25)] = 351,
  [SMALL_STATE(26)] = 367,
  [SMALL_STATE(27)] = 383,
  [SMALL_STATE(28)] = 399,
  [SMALL_STATE(29)] = 413,
  [SMALL_STATE(30)] = 426,
  [SMALL_STATE(31)] = 437,
  [SMALL_STATE(32)] = 450,
  [SMALL_STATE(33)] = 460,
  [SMALL_STATE(34)] = 468,
  [SMALL_STATE(35)] = 476,
  [SMALL_STATE(36)] = 486,
  [SMALL_STATE(37)] = 496,
  [SMALL_STATE(38)] = 506,
  [SMALL_STATE(39)] = 516,
  [SMALL_STATE(40)] = 524,
  [SMALL_STATE(41)] = 531,
  [SMALL_STATE(42)] = 538,
  [SMALL_STATE(43)] = 545,
  [SMALL_STATE(44)] = 552,
  [SMALL_STATE(45)] = 559,
  [SMALL_STATE(46)] = 566,
  [SMALL_STATE(47)] = 570,
  [SMALL_STATE(48)] = 574,
  [SMALL_STATE(49)] = 578,
  [SMALL_STATE(50)] = 582,
  [SMALL_STATE(51)] = 586,
  [SMALL_STATE(52)] = 590,
  [SMALL_STATE(53)] = 594,
  [SMALL_STATE(54)] = 598,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(2),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(52),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(11),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(47),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [48] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__line, 2, 0, 0),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 2, 0, 5),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 2, 0, 5),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 2, 0, 4),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 2, 0, 4),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 13),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 13),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 1, 0, 1),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 1, 0, 1),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_instruction_repeat1, 2, 0, 0),
  [80] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_instruction_repeat1, 2, 0, 0), SHIFT_REPEAT(12),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 2, 0, 2),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 2, 0, 2),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 2, 0, 3),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 2, 0, 3),
  [93] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__args, 2, 0, 6), SHIFT_REPEAT(8),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__args, 2, 0, 6),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__args, 2, 0, 6),
  [100] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arg, 1, 0, 0),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arg, 1, 0, 0),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 10),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 10),
  [110] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 11),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 11),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 12),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 12),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 8),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 8),
  [122] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 9),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 9),
  [126] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 4, 0, 14),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 4, 0, 14),
  [130] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_arg_repeat1, 2, 0, 0), SHIFT_REPEAT(23),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arg_repeat1, 2, 0, 0),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_arg_repeat1, 2, 0, 0),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 4, 0, 15),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 4, 0, 15),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 4, 0, 17),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 4, 0, 17),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 4, 0, 16),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 4, 0, 16),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 5, 0, 18),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 5, 0, 18),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_instruction_repeat1, 2, 0, 0),
  [155] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_instruction_repeat1, 2, 0, 0), SHIFT_REPEAT(28),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [160] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_goto, 1, 0, 0),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_goto, 1, 0, 0),
  [164] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [168] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_instruction_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__output, 4, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__label, 2, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__label, 2, 0, 0),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__instruction_label, 2, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__output, 3, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__output, 2, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__args, 2, 0, 7),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__args, 2, 0, 7),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comment, 3, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comment, 3, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comment, 2, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comment, 2, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive, 1, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive, 1, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [213] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_crabscript(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
