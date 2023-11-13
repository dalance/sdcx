use crate::parser::sdc_grammar_trait as grammar;
use parol_runtime::ParolError;
use std::fmt;
use thiserror::Error;

pub mod argument;
pub mod command;
pub mod util;
pub use argument::Argument;
pub use command::*;

/// SDC Error
#[derive(Debug, Error)]
pub enum SdcError {
    #[error("WrongArgument: {0:?}")]
    WrongArgument(Vec<Argument>),

    #[error("UnknownCommand: {0}")]
    UnknownCommand(String),

    #[error("DuplicatedArgument")]
    DuplicatedArgument(Argument),

    #[error("MissingOptArgument: {0:?}")]
    MissingOptArgument(Argument),

    #[error("MissingPosArgument")]
    MissingPosArgument,

    #[error("TooManyArgument")]
    TooManyArgument,

    #[error("MissingMandatoryArgument: {0}")]
    MissingMandatoryArgument(String),

    #[error("ParseError: {0}")]
    ParseError(#[from] ParolError),

    #[error("SdcVersionPlacement")]
    SdcVersionPlacement,

    #[error("UnknownVersion")]
    UnknownVersion,
}

/// SDC
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sdc {
    pub header: Vec<String>,
    pub version: Option<SdcVersion>,
    pub commands: Vec<Command>,
}

impl Sdc {
    pub fn validate(&self) -> bool {
        let version = self.version.unwrap_or(SdcVersion::SDC2_1);
        self.commands.iter().all(|x| x.validate(version))
    }
}

impl TryFrom<&grammar::Source<'_>> for Sdc {
    type Error = SdcError;

    fn try_from(value: &grammar::Source<'_>) -> Result<Self, SdcError> {
        let mut sdc = Sdc::default();
        let mut is_header = true;
        let mut is_first_command = true;
        for source in &value.source_list {
            match source.source_list_group.as_ref() {
                grammar::SourceListGroup::CommandLine(x) => {
                    is_header = false;
                    let command = x.command_line.command.as_ref().try_into()?;

                    match command {
                        Command::Set(x) if x.variable_name == "sdc_version".into() => {
                            if is_first_command {
                                match x.value.as_str() {
                                    "1.1" => sdc.version = Some(SdcVersion::SDC1_1),
                                    "1.2" => sdc.version = Some(SdcVersion::SDC1_2),
                                    "1.3" => sdc.version = Some(SdcVersion::SDC1_3),
                                    "1.4" => sdc.version = Some(SdcVersion::SDC1_4),
                                    "1.5" => sdc.version = Some(SdcVersion::SDC1_5),
                                    "1.6" => sdc.version = Some(SdcVersion::SDC1_6),
                                    "1.7" => sdc.version = Some(SdcVersion::SDC1_7),
                                    "1.8" => sdc.version = Some(SdcVersion::SDC1_8),
                                    "1.9" => sdc.version = Some(SdcVersion::SDC1_9),
                                    "2.0" => sdc.version = Some(SdcVersion::SDC2_0),
                                    "2.1" => sdc.version = Some(SdcVersion::SDC2_1),
                                    _ => return Err(SdcError::UnknownVersion),
                                }
                            } else {
                                return Err(SdcError::SdcVersionPlacement);
                            }
                        }
                        _ => sdc.commands.push(command),
                    }

                    if is_first_command {
                        is_first_command = false;
                    }
                }
                grammar::SourceListGroup::TermComment(x) => {
                    if is_header {
                        sdc.header.push(x.term_comment.term_comment.text().into());
                    }
                }
                _ => (),
            }
        }
        Ok(sdc)
    }
}

/// SDC version
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SdcVersion {
    SDC1_1 = 0,
    SDC1_2 = 1,
    SDC1_3 = 2,
    SDC1_4 = 3,
    SDC1_5 = 4,
    SDC1_6 = 5,
    SDC1_7 = 6,
    SDC1_8 = 7,
    SDC1_9 = 8,
    SDC2_0 = 9,
    SDC2_1 = 10,
}

impl fmt::Display for SdcVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdcVersion::SDC1_1 => "set sdc_version 1.1".fmt(f),
            SdcVersion::SDC1_2 => "set sdc_version 1.2".fmt(f),
            SdcVersion::SDC1_3 => "set sdc_version 1.3".fmt(f),
            SdcVersion::SDC1_4 => "set sdc_version 1.4".fmt(f),
            SdcVersion::SDC1_5 => "set sdc_version 1.5".fmt(f),
            SdcVersion::SDC1_6 => "set sdc_version 1.6".fmt(f),
            SdcVersion::SDC1_7 => "set sdc_version 1.7".fmt(f),
            SdcVersion::SDC1_8 => "set sdc_version 1.8".fmt(f),
            SdcVersion::SDC1_9 => "set sdc_version 1.9".fmt(f),
            SdcVersion::SDC2_0 => "set sdc_version 2.0".fmt(f),
            SdcVersion::SDC2_1 => "set sdc_version 2.1".fmt(f),
        }
    }
}

trait Validate {
    fn validate(&self, version: SdcVersion) -> bool;
}
