use crate::parser::sdc_grammar_trait as grammar;
use crate::sdc::{Command, Location, SdcError};
use std::fmt;

/// Argument
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Argument {
    Word(ArgumentWord),
    StringGroup(ArgumentStringGroup),
    BraceGroup(ArgumentBraceGroup),
    CommandReplacement(Box<Command>, Location),
}

impl Argument {
    pub fn as_str(&self) -> &str {
        match self {
            Argument::Word(x) => x.text.as_str(),
            Argument::StringGroup(x) => x.text.as_str(),
            Argument::BraceGroup(x) => x.text.as_str(),
            Argument::CommandReplacement(_, _) => "",
        }
    }

    pub fn location(&self) -> &Location {
        match self {
            Argument::Word(x) => &x.location,
            Argument::StringGroup(x) => &x.location,
            Argument::BraceGroup(x) => &x.location,
            Argument::CommandReplacement(_, x) => x,
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Word(x) => x.text.fmt(f),
            Argument::StringGroup(x) => x.text.fmt(f),
            Argument::BraceGroup(x) => x.text.fmt(f),
            Argument::CommandReplacement(x, _) => format!("[{}]", x).fmt(f),
        }
    }
}

impl From<&str> for Argument {
    fn from(value: &str) -> Self {
        let location = Location::default();
        Argument::Word(ArgumentWord {
            text: value.into(),
            location,
        })
    }
}

impl TryFrom<&grammar::Argument<'_>> for Argument {
    type Error = SdcError;

    fn try_from(value: &grammar::Argument) -> Result<Self, SdcError> {
        match value {
            grammar::Argument::TokenWord(x) => {
                let text = x.token_word.term_word.term_word.text().to_string();
                let location = (&x.token_word.term_word.term_word.location).into();
                Ok(Self::Word(ArgumentWord { text, location }))
            }
            grammar::Argument::TokenStringGroup(x) => {
                let text = x
                    .token_string_group
                    .term_string_group
                    .term_string_group
                    .text()
                    .to_string();
                let location = (&x
                    .token_string_group
                    .term_string_group
                    .term_string_group
                    .location)
                    .into();

                Ok(Self::StringGroup(ArgumentStringGroup { text, location }))
            }
            grammar::Argument::TokenBraceGroup(x) => {
                let text = x.token_brace_group.term_brace_group.to_string();
                let start: Location = (&x
                    .token_brace_group
                    .term_brace_group
                    .term_l_brace
                    .term_l_brace
                    .location)
                    .into();
                let end: Location = (&x
                    .token_brace_group
                    .term_brace_group
                    .term_r_brace
                    .term_r_brace
                    .location)
                    .into();
                let location = Location::from_to(&start, &end);

                Ok(Self::StringGroup(ArgumentStringGroup { text, location }))
            }
            grammar::Argument::CommandReplacement(x) => {
                let start: Location = (&x
                    .command_replacement
                    .token_l_bracket
                    .term_l_bracket
                    .term_l_bracket
                    .location)
                    .into();
                let end: Location = (&x
                    .command_replacement
                    .token_r_bracket
                    .term_r_bracket
                    .term_r_bracket
                    .location)
                    .into();
                let location = Location::from_to(&start, &end);

                Ok(Self::CommandReplacement(
                    Box::new(x.command_replacement.command.as_ref().try_into()?),
                    location,
                ))
            }
        }
    }
}

impl ToString for grammar::TermBraceGroup<'_> {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(self.term_l_brace.term_l_brace.text());
        match self.term_brace_group_group.as_ref() {
            grammar::TermBraceGroupGroup::TermBraceGroup(x) => {
                ret.push_str(&x.term_brace_group.to_string());
            }
            grammar::TermBraceGroupGroup::TermBraceGroupContent(x) => {
                ret.push_str(x.term_brace_group_content.term_brace_group_content.text());
            }
        }
        ret.push_str(self.term_r_brace.term_r_brace.text());
        ret
    }
}

/// ArgumentWord
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentWord {
    pub text: String,
    location: Location,
}

/// ArgumentStringGroup
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentStringGroup {
    pub text: String,
    location: Location,
}

/// ArgumentBraceGroup
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentBraceGroup {
    pub text: String,
    location: Location,
}
