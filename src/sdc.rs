use crate::errors::{SemanticError, ValidateError};
use crate::parser::sdc_grammar_trait as grammar;
use crate::sdc::util::Validate;
use std::fmt;

pub mod argument;
pub mod command;
pub mod sdc_error;
pub mod util;
pub use argument::Argument;
pub use command::*;

/// SDC
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sdc {
    pub header: Vec<String>,
    pub version: Option<SdcVersion>,
    pub commands: Vec<Command>,
}

impl Sdc {
    pub fn validate(&self, force_version: Option<SdcVersion>) -> Vec<ValidateError> {
        let version = self.version.unwrap_or(SdcVersion::SDC2_1);
        let version = force_version.unwrap_or(version);
        let mut ret = vec![];
        for command in &self.commands {
            ret.append(&mut command.validate(version));
        }
        ret
    }

    pub fn normalize(&mut self) {
        self.commands.sort()
    }
}

impl fmt::Display for Sdc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.header {
            write!(f, "{}", s)?;
        }
        if let Some(version) = self.version {
            writeln!(f, "{}", version)?;
        }
        for c in &self.commands {
            writeln!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl TryFrom<&grammar::Source<'_>> for Sdc {
    type Error = SemanticError;

    fn try_from(value: &grammar::Source<'_>) -> Result<Self, SemanticError> {
        let mut sdc = Sdc::default();
        let mut is_header = true;
        let mut is_first_command = true;
        for source in &value.source_list {
            match source.source_list_group.as_ref() {
                grammar::SourceListGroup::CommandLine(x) => {
                    is_header = false;
                    let command = x.command_line.command.as_ref().try_into()?;

                    match command {
                        Command::Set(x) if x.variable_name.as_str() == "sdc_version" => {
                            if is_first_command {
                                if let Ok(sdc_version) = x.value.as_str().try_into() {
                                    sdc.version = Some(sdc_version);
                                } else {
                                    return Err(SemanticError::UnknownVersion(
                                        x.location().clone(),
                                    ));
                                }
                            } else {
                                return Err(SemanticError::SdcVersionPlacement(
                                    x.location().clone(),
                                ));
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

impl SdcVersion {
    pub fn within(&self, from: SdcVersion, to: SdcVersion) -> (bool, SdcVersion) {
        (&from <= self && self <= &to, *self)
    }

    pub fn version_string(&self) -> &str {
        match self {
            SdcVersion::SDC1_1 => "1.1",
            SdcVersion::SDC1_2 => "1.2",
            SdcVersion::SDC1_3 => "1.3",
            SdcVersion::SDC1_4 => "1.4",
            SdcVersion::SDC1_5 => "1.5",
            SdcVersion::SDC1_6 => "1.6",
            SdcVersion::SDC1_7 => "1.7",
            SdcVersion::SDC1_8 => "1.8",
            SdcVersion::SDC1_9 => "1.9",
            SdcVersion::SDC2_0 => "2.0",
            SdcVersion::SDC2_1 => "2.1",
        }
    }
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

impl TryFrom<&str> for SdcVersion {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1.1" => Ok(SdcVersion::SDC1_1),
            "1.2" => Ok(SdcVersion::SDC1_2),
            "1.3" => Ok(SdcVersion::SDC1_3),
            "1.4" => Ok(SdcVersion::SDC1_4),
            "1.5" => Ok(SdcVersion::SDC1_5),
            "1.6" => Ok(SdcVersion::SDC1_6),
            "1.7" => Ok(SdcVersion::SDC1_7),
            "1.8" => Ok(SdcVersion::SDC1_8),
            "1.9" => Ok(SdcVersion::SDC1_9),
            "2.0" => Ok(SdcVersion::SDC2_0),
            "2.1" => Ok(SdcVersion::SDC2_1),
            _ => Err(()),
        }
    }
}
