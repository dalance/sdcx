// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{TokenStream, Tokenizer};
use std::path::Path;

use crate::parser::sdc_grammar::SdcGrammar;
use crate::parser::sdc_grammar_trait::SdcGrammarAuto;

use parol_runtime::lexer::tokenizer::{ERROR_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN};

pub const TERMINALS: &[&str; 17] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r"\[",
    /*  6 */ r"\]",
    /*  7 */ r"\{",
    /*  8 */ r"\}",
    /*  9 */ r"\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\]|\\\n)*\u{0022}",
    /* 10 */ r"#.*(\r\n|\r|\n|$)",
    /* 11 */ r";",
    /* 12 */ r"\\(\r\n|\r|\n)",
    /* 13 */ r"(\r\n|\r|\n|$)",
    /* 14 */ r"[^\s\[\]\\;]+",
    /* 15 */ r"[^}]*",
    /* 16 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 17] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "TermLBracket",
    /*  6 */ "TermRBracket",
    /*  7 */ "TermLBrace",
    /*  8 */ "TermRBrace",
    /*  9 */ "TermStringGroup",
    /* 10 */ "TermComment",
    /* 11 */ "TermSemiColon",
    /* 12 */ "TermBackslashLineBreak",
    /* 13 */ "TermLineBreak",
    /* 14 */ "TermWord",
    /* 15 */ "TermBraceGroupContent",
    /* 16 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 10]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* TermLBracket */
        6,  /* TermRBracket */
        7,  /* TermLBrace */
        8,  /* TermRBrace */
        9,  /* TermStringGroup */
        10, /* TermComment */
        11, /* TermSemiColon */
        12, /* TermBackslashLineBreak */
        13, /* TermLineBreak */
        14, /* TermWord */
    ],
);

/* SCANNER_1: "BraceGroup" */
const SCANNER_1: (&[&str; 5], &[TerminalIndex; 3]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ UNMATCHABLE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        7,  /* TermLBrace */
        8,  /* TermRBrace */
        15, /* TermBraceGroupContent */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 32] = &[
    /*  0 */ "Argument",
    /*  1 */ "Command",
    /*  2 */ "CommandLine",
    /*  3 */ "CommandList",
    /*  4 */ "CommandReplacement",
    /*  5 */ "Source",
    /*  6 */ "SourceList",
    /*  7 */ "SourceListGroup",
    /*  8 */ "TermBackslashLineBreak",
    /*  9 */ "TermBraceGroup",
    /* 10 */ "TermBraceGroupContent",
    /* 11 */ "TermBraceGroupGroup",
    /* 12 */ "TermComment",
    /* 13 */ "TermLBrace",
    /* 14 */ "TermLBracket",
    /* 15 */ "TermLineBreak",
    /* 16 */ "TermRBrace",
    /* 17 */ "TermRBracket",
    /* 18 */ "TermSemiColon",
    /* 19 */ "TermStringGroup",
    /* 20 */ "TermWord",
    /* 21 */ "TokenBraceGroup",
    /* 22 */ "TokenBraceGroupOpt",
    /* 23 */ "TokenEnd",
    /* 24 */ "TokenLBracket",
    /* 25 */ "TokenLBracketOpt",
    /* 26 */ "TokenRBracket",
    /* 27 */ "TokenRBracketOpt",
    /* 28 */ "TokenStringGroup",
    /* 29 */ "TokenStringGroupOpt",
    /* 30 */ "TokenWord",
    /* 31 */ "TokenWordOpt",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 32] = &[
    /* 0 - "Argument" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 4, 34),
            Trans(0, 7, 3, 33),
            Trans(0, 9, 2, 32),
            Trans(0, 14, 1, 31),
        ],
        k: 1,
    },
    /* 1 - "Command" */
    LookaheadDFA {
        prod0: 36,
        transitions: &[],
        k: 0,
    },
    /* 2 - "CommandLine" */
    LookaheadDFA {
        prod0: 39,
        transitions: &[],
        k: 0,
    },
    /* 3 - "CommandList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 1, 37),
            Trans(0, 6, 2, 38),
            Trans(0, 7, 1, 37),
            Trans(0, 9, 1, 37),
            Trans(0, 11, 2, 38),
            Trans(0, 13, 2, 38),
            Trans(0, 14, 1, 37),
        ],
        k: 1,
    },
    /* 4 - "CommandReplacement" */
    LookaheadDFA {
        prod0: 35,
        transitions: &[],
        k: 0,
    },
    /* 5 - "Source" */
    LookaheadDFA {
        prod0: 40,
        transitions: &[],
        k: 0,
    },
    /* 6 - "SourceList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 45),
            Trans(0, 10, 1, 41),
            Trans(0, 11, 1, 41),
            Trans(0, 13, 1, 41),
            Trans(0, 14, 1, 41),
        ],
        k: 1,
    },
    /* 7 - "SourceListGroup" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 3, 44),
            Trans(0, 11, 2, 43),
            Trans(0, 13, 2, 43),
            Trans(0, 14, 1, 42),
        ],
        k: 1,
    },
    /* 8 - "TermBackslashLineBreak" */
    LookaheadDFA {
        prod0: 7,
        transitions: &[],
        k: 0,
    },
    /* 9 - "TermBraceGroup" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
    /* 10 - "TermBraceGroupContent" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 11 - "TermBraceGroupGroup" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 7, 1, 12), Trans(0, 15, 2, 13)],
        k: 1,
    },
    /* 12 - "TermComment" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 13 - "TermLBrace" */
    LookaheadDFA {
        prod0: 2,
        transitions: &[],
        k: 0,
    },
    /* 14 - "TermLBracket" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 15 - "TermLineBreak" */
    LookaheadDFA {
        prod0: 8,
        transitions: &[],
        k: 0,
    },
    /* 16 - "TermRBrace" */
    LookaheadDFA {
        prod0: 3,
        transitions: &[],
        k: 0,
    },
    /* 17 - "TermRBracket" */
    LookaheadDFA {
        prod0: 1,
        transitions: &[],
        k: 0,
    },
    /* 18 - "TermSemiColon" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 19 - "TermStringGroup" */
    LookaheadDFA {
        prod0: 4,
        transitions: &[],
        k: 0,
    },
    /* 20 - "TermWord" */
    LookaheadDFA {
        prod0: 9,
        transitions: &[],
        k: 0,
    },
    /* 21 - "TokenBraceGroup" */
    LookaheadDFA {
        prod0: 16,
        transitions: &[],
        k: 0,
    },
    /* 22 - "TokenBraceGroupOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 18),
            Trans(0, 6, 2, 18),
            Trans(0, 7, 2, 18),
            Trans(0, 9, 2, 18),
            Trans(0, 11, 2, 18),
            Trans(0, 12, 1, 17),
            Trans(0, 13, 2, 18),
            Trans(0, 14, 2, 18),
        ],
        k: 1,
    },
    /* 23 - "TokenEnd" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 11, 2, 15), Trans(0, 13, 1, 14)],
        k: 1,
    },
    /* 24 - "TokenLBracket" */
    LookaheadDFA {
        prod0: 22,
        transitions: &[],
        k: 0,
    },
    /* 25 - "TokenLBracketOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 12, 1, 23), Trans(0, 14, 2, 24)],
        k: 1,
    },
    /* 26 - "TokenRBracket" */
    LookaheadDFA {
        prod0: 25,
        transitions: &[],
        k: 0,
    },
    /* 27 - "TokenRBracketOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 27),
            Trans(0, 6, 2, 27),
            Trans(0, 7, 2, 27),
            Trans(0, 9, 2, 27),
            Trans(0, 11, 2, 27),
            Trans(0, 12, 1, 26),
            Trans(0, 13, 2, 27),
            Trans(0, 14, 2, 27),
        ],
        k: 1,
    },
    /* 28 - "TokenStringGroup" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 29 - "TokenStringGroupOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 21),
            Trans(0, 6, 2, 21),
            Trans(0, 7, 2, 21),
            Trans(0, 9, 2, 21),
            Trans(0, 11, 2, 21),
            Trans(0, 12, 1, 20),
            Trans(0, 13, 2, 21),
            Trans(0, 14, 2, 21),
        ],
        k: 1,
    },
    /* 30 - "TokenWord" */
    LookaheadDFA {
        prod0: 28,
        transitions: &[],
        k: 0,
    },
    /* 31 - "TokenWordOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 30),
            Trans(0, 6, 2, 30),
            Trans(0, 7, 2, 30),
            Trans(0, 9, 2, 30),
            Trans(0, 11, 2, 30),
            Trans(0, 12, 1, 29),
            Trans(0, 13, 2, 30),
            Trans(0, 14, 2, 30),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 46] = &[
    // 0 - TermLBracket: '[';
    Production {
        lhs: 14,
        production: &[ParseType::T(5)],
    },
    // 1 - TermRBracket: ']';
    Production {
        lhs: 17,
        production: &[ParseType::T(6)],
    },
    // 2 - TermLBrace: '{';
    Production {
        lhs: 13,
        production: &[ParseType::T(7)],
    },
    // 3 - TermRBrace: '}';
    Production {
        lhs: 16,
        production: &[ParseType::T(8)],
    },
    // 4 - TermStringGroup: "\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\]|\\\n)*\u{0022}";
    Production {
        lhs: 19,
        production: &[ParseType::T(9)],
    },
    // 5 - TermComment: /#.*(\r\n|\r|\n|$)/;
    Production {
        lhs: 12,
        production: &[ParseType::T(10)],
    },
    // 6 - TermSemiColon: ';';
    Production {
        lhs: 18,
        production: &[ParseType::T(11)],
    },
    // 7 - TermBackslashLineBreak: /\\(\r\n|\r|\n)/;
    Production {
        lhs: 8,
        production: &[ParseType::T(12)],
    },
    // 8 - TermLineBreak: /(\r\n|\r|\n|$)/;
    Production {
        lhs: 15,
        production: &[ParseType::T(13)],
    },
    // 9 - TermWord: /[^\s\[\]\\;]+/;
    Production {
        lhs: 20,
        production: &[ParseType::T(14)],
    },
    // 10 - TermBraceGroupContent: /[^}]*/;
    Production {
        lhs: 10,
        production: &[ParseType::T(15)],
    },
    // 11 - TermBraceGroup: TermLBrace Push(1) TermBraceGroupGroup TermRBrace Pop;
    Production {
        lhs: 9,
        production: &[
            ParseType::Pop,
            ParseType::N(16),
            ParseType::N(11),
            ParseType::Push(1),
            ParseType::N(13),
        ],
    },
    // 12 - TermBraceGroupGroup: TermBraceGroup;
    Production {
        lhs: 11,
        production: &[ParseType::N(9)],
    },
    // 13 - TermBraceGroupGroup: TermBraceGroupContent;
    Production {
        lhs: 11,
        production: &[ParseType::N(10)],
    },
    // 14 - TokenEnd: TermLineBreak;
    Production {
        lhs: 23,
        production: &[ParseType::N(15)],
    },
    // 15 - TokenEnd: TermSemiColon;
    Production {
        lhs: 23,
        production: &[ParseType::N(18)],
    },
    // 16 - TokenBraceGroup: TermBraceGroup TokenBraceGroupOpt /* Option */;
    Production {
        lhs: 21,
        production: &[ParseType::N(22), ParseType::N(9)],
    },
    // 17 - TokenBraceGroupOpt: TermBackslashLineBreak;
    Production {
        lhs: 22,
        production: &[ParseType::N(8)],
    },
    // 18 - TokenBraceGroupOpt: ;
    Production {
        lhs: 22,
        production: &[],
    },
    // 19 - TokenStringGroup: TermStringGroup TokenStringGroupOpt /* Option */;
    Production {
        lhs: 28,
        production: &[ParseType::N(29), ParseType::N(19)],
    },
    // 20 - TokenStringGroupOpt: TermBackslashLineBreak;
    Production {
        lhs: 29,
        production: &[ParseType::N(8)],
    },
    // 21 - TokenStringGroupOpt: ;
    Production {
        lhs: 29,
        production: &[],
    },
    // 22 - TokenLBracket: TermLBracket TokenLBracketOpt /* Option */;
    Production {
        lhs: 24,
        production: &[ParseType::N(25), ParseType::N(14)],
    },
    // 23 - TokenLBracketOpt: TermBackslashLineBreak;
    Production {
        lhs: 25,
        production: &[ParseType::N(8)],
    },
    // 24 - TokenLBracketOpt: ;
    Production {
        lhs: 25,
        production: &[],
    },
    // 25 - TokenRBracket: TermRBracket TokenRBracketOpt /* Option */;
    Production {
        lhs: 26,
        production: &[ParseType::N(27), ParseType::N(17)],
    },
    // 26 - TokenRBracketOpt: TermBackslashLineBreak;
    Production {
        lhs: 27,
        production: &[ParseType::N(8)],
    },
    // 27 - TokenRBracketOpt: ;
    Production {
        lhs: 27,
        production: &[],
    },
    // 28 - TokenWord: TermWord TokenWordOpt /* Option */;
    Production {
        lhs: 30,
        production: &[ParseType::N(31), ParseType::N(20)],
    },
    // 29 - TokenWordOpt: TermBackslashLineBreak;
    Production {
        lhs: 31,
        production: &[ParseType::N(8)],
    },
    // 30 - TokenWordOpt: ;
    Production {
        lhs: 31,
        production: &[],
    },
    // 31 - Argument: TokenWord;
    Production {
        lhs: 0,
        production: &[ParseType::N(30)],
    },
    // 32 - Argument: TokenStringGroup;
    Production {
        lhs: 0,
        production: &[ParseType::N(28)],
    },
    // 33 - Argument: TokenBraceGroup;
    Production {
        lhs: 0,
        production: &[ParseType::N(21)],
    },
    // 34 - Argument: CommandReplacement;
    Production {
        lhs: 0,
        production: &[ParseType::N(4)],
    },
    // 35 - CommandReplacement: TokenLBracket Command TokenRBracket;
    Production {
        lhs: 4,
        production: &[ParseType::N(26), ParseType::N(1), ParseType::N(24)],
    },
    // 36 - Command: TokenWord CommandList /* Vec */;
    Production {
        lhs: 1,
        production: &[ParseType::N(3), ParseType::N(30)],
    },
    // 37 - CommandList: Argument CommandList;
    Production {
        lhs: 3,
        production: &[ParseType::N(3), ParseType::N(0)],
    },
    // 38 - CommandList: ;
    Production {
        lhs: 3,
        production: &[],
    },
    // 39 - CommandLine: Command TokenEnd;
    Production {
        lhs: 2,
        production: &[ParseType::N(23), ParseType::N(1)],
    },
    // 40 - Source: SourceList /* Vec */;
    Production {
        lhs: 5,
        production: &[ParseType::N(6)],
    },
    // 41 - SourceList: SourceListGroup SourceList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(7)],
    },
    // 42 - SourceListGroup: CommandLine;
    Production {
        lhs: 7,
        production: &[ParseType::N(2)],
    },
    // 43 - SourceListGroup: TokenEnd;
    Production {
        lhs: 7,
        production: &[ParseType::N(23)],
    },
    // 44 - SourceListGroup: TermComment;
    Production {
        lhs: 7,
        production: &[ParseType::N(12)],
    },
    // 45 - SourceList: ;
    Production {
        lhs: 6,
        production: &[],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![
        (
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
        ),
        (
            "BraceGroup",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut SdcGrammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        5,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.trim_parse_tree();
    // Initialize wrapper
    let mut user_actions = SdcGrammarAuto::new(user_actions);

    llk_parser.parse(
        TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
