use crate::parser::sdc_grammar_trait as grammar;
use crate::sdc::sdc_error::FileDb;
use crate::sdc::util::Validate;
use std::fmt;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

pub mod argument;
pub mod command;
pub mod sdc_error;
pub mod util;
pub use argument::Argument;
pub use command::*;
pub use sdc_error::SdcError;

/// SDC
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sdc {
    pub header: Vec<String>,
    pub version: Option<SdcVersion>,
    pub commands: Vec<Command>,
}

impl Sdc {
    pub fn validate(&self) -> Result<(), SdcError> {
        let version = self.version.unwrap_or(SdcVersion::SDC2_1);
        for command in &self.commands {
            command.validate(version)?;
        }
        Ok(())
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
            write!(f, "{}\n", version)?;
        }
        for c in &self.commands {
            write!(f, "{}\n", c)?;
        }

        Ok(())
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
                        Command::Set(x) if x.variable_name.as_str() == "sdc_version" => {
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

impl SdcVersion {
    pub fn within(&self, from: SdcVersion, to: SdcVersion) -> (bool, SdcVersion) {
        (&from <= self && self <= &to, *self)
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

/// Location
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub start_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub length: u32,
    pub file_name: Arc<PathBuf>,
}

impl Location {
    fn from_to(from: &Location, to: &Location) -> Location {
        Location {
            start_byte: from.start_byte,
            start_line: from.start_line,
            start_column: to.start_column,
            end_line: to.end_line,
            end_column: to.end_column,
            length: to.start_byte - from.start_byte + to.length,
            file_name: from.file_name.clone(),
        }
    }

    fn range_file(&self, files: &FileDb<String, &str>) -> (Range<usize>, usize) {
        let range: Range<usize> = self.into();
        let file_id = files.get_id(&self.file_name.display().to_string()).unwrap();
        (range, file_id)
    }
}

impl From<&Location> for Range<usize> {
    fn from(value: &Location) -> Self {
        Range {
            start: value.start_byte as usize,
            end: (value.start_byte + value.length) as usize,
        }
    }
}

impl From<&parol_runtime::Location> for Location {
    fn from(value: &parol_runtime::Location) -> Self {
        let start_byte = value.scanner_switch_pos as u32 + value.offset as u32 - value.length;
        Location {
            start_byte,
            start_line: value.start_line,
            start_column: value.start_column,
            end_line: value.end_line,
            end_column: value.end_column,
            length: value.length,
            file_name: value.file_name.clone(),
        }
    }
}
