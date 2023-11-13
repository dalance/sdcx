use crate::parser::sdc_grammar_trait as grammar;
use crate::sdc::{Command, SdcError};
use std::fmt;

/// Argument
#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    Word(String),
    StringGroup(String),
    BraceGroup(String),
    CommandReplacement(Box<Command>),
}

impl Argument {
    pub fn as_str(&self) -> &str {
        match self {
            Argument::Word(x) => x.as_str(),
            Argument::StringGroup(x) => x.as_str(),
            Argument::BraceGroup(x) => x.as_str(),
            Argument::CommandReplacement(_) => "",
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Word(x) => x.fmt(f),
            Argument::StringGroup(x) => x.fmt(f),
            Argument::BraceGroup(x) => x.fmt(f),
            Argument::CommandReplacement(x) => format!("[{}]", x).fmt(f),
        }
    }
}

impl From<&str> for Argument {
    fn from(value: &str) -> Self {
        Argument::Word(value.into())
    }
}

impl TryFrom<&grammar::Argument<'_>> for Argument {
    type Error = SdcError;

    fn try_from(value: &grammar::Argument) -> Result<Self, SdcError> {
        match value {
            grammar::Argument::TokenWord(x) => Ok(Self::Word(
                x.token_word.term_word.term_word.text().to_string(),
            )),
            grammar::Argument::TokenStringGroup(x) => Ok(Self::StringGroup(
                x.token_string_group
                    .term_string_group
                    .term_string_group
                    .text()
                    .to_string(),
            )),
            grammar::Argument::TokenBraceGroup(x) => Ok(Self::BraceGroup(
                x.token_brace_group
                    .term_brace_group
                    .term_brace_group
                    .text()
                    .to_string(),
            )),
            grammar::Argument::CommandReplacement(x) => Ok(Self::CommandReplacement(Box::new(
                x.command_replacement.command.as_ref().try_into()?,
            ))),
        }
    }
}
