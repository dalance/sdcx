use crate::sdc::{Argument, Location, SdcVersion};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{self, Files, SimpleFile};
use codespan_reporting::term::{self, termcolor::StandardStream};
use parol_runtime::{LexerError, ParolError, ParserError, Span, SyntaxError};
use std::collections::HashMap;
use std::ops::Range;
use thiserror::Error;

/// SDC Error
#[derive(Debug, Error)]
pub enum SdcError {
    #[error("WrongArgument: {0:?}")]
    WrongArgument(Argument),

    #[error("UnknownCommand: {0}")]
    UnknownCommand(String, Location),

    #[error("DuplicatedArgument")]
    DuplicatedArgument(Argument),

    #[error("MissingOptArgument: {0:?}")]
    MissingOptArgument(Argument),

    #[error("MissingPosArgument")]
    MissingPosArgument(Location),

    #[error("TooManyArgument")]
    TooManyArgument(Location),

    #[error("MissingMandatoryArgument: {0}")]
    MissingMandatoryArgument(String, Location),

    #[error("ParseError: {0}")]
    ParseError(#[from] ParolError),

    #[error("SdcVersionPlacement")]
    SdcVersionPlacement(Location),

    #[error("UnknownVersion")]
    UnknownVersion(Location),

    #[error("CmdUnsupportedVersion")]
    CmdUnsupportedVersion(SdcVersion, Location),

    #[error("ArgUnsupportedVersion")]
    ArgUnsupportedVersion(SdcVersion, Location, String),

    #[error("ArgumentCombination")]
    ArgumentCombination(Location),

    #[error("AmbiguousOption")]
    AmbiguousOption(Location),

    #[error("GenaralError")]
    GenaralError(#[from] anyhow::Error),
}

impl SdcError {
    pub fn report(self, files: &FileDb<String, &str>) -> anyhow::Result<()> {
        let writer = StandardStream::stderr(term::termcolor::ColorChoice::Auto);
        let config = term::Config::default();

        match self {
            SdcError::ParseError(ParolError::LexerError(x)) => {
                Self::report_lexer_error(&x, &writer, &config, &files)
            }
            SdcError::ParseError(ParolError::ParserError(x)) => {
                Self::report_parser_error(&x, &writer, &config, &files)
            }
            SdcError::ParseError(ParolError::UserError(_)) => Ok(()),
            SdcError::WrongArgument(x) => {
                let (range, file_id) = x.location().range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Wrong argument")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::UnknownCommand(_, location) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Unknown command")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::DuplicatedArgument(x) => {
                let (range, file_id) = x.location().range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Duplicated arguments")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::MissingOptArgument(x) => {
                let (range, file_id) = x.location().range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Missing argument")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::MissingPosArgument(x) => {
                let (range, file_id) = x.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Missing positional argument")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::TooManyArgument(x) => {
                let (range, file_id) = x.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message("Too many argument")
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::MissingMandatoryArgument(name, location) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!("Missing mandatory argument: {name}"))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::SdcVersionPlacement(location) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!(
                        "SDC version should be set at the beginning of file"
                    ))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::UnknownVersion(location) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!("Unknown SDC version"))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::CmdUnsupportedVersion(version, location) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!(
                        "Unsupported command at SDC {}",
                        version.version_string()
                    ))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::ArgUnsupportedVersion(version, location, name) => {
                let (range, file_id) = location.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!(
                        "Unsupported argument \"-{name}\" at {}",
                        version.version_string()
                    ))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::ArgumentCombination(x) => {
                let (range, file_id) = x.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!("Forbidden argument combination"))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::AmbiguousOption(x) => {
                let (range, file_id) = x.range_file(&files);
                let diag = Diagnostic::error()
                    .with_message(format!("Ambiguous option"))
                    .with_code("sdcx::sdc::SdcError")
                    .with_labels(vec![Label::primary(file_id, range).with_message("Found")]);
                Ok(term::emit(&mut writer.lock(), &config, files, &diag)?)
            }
            SdcError::GenaralError(x) => Err(x.into()),
        }
    }

    fn report_lexer_error(
        err: &LexerError,
        writer: &StandardStream,
        config: &term::Config,
        files: &FileDb<String, &str>,
    ) -> anyhow::Result<()> {
        match err {
            LexerError::TokenBufferEmptyError => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message("No valid token read")
                    .with_code("parol_runtime::lexer::empty_token_buffer")
                    .with_notes(vec!["Token buffer is empty".to_string()]),
            )?),
            LexerError::InternalError(e) => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message(format!("Internal lexer error: {e}"))
                    .with_code("parol_runtime::lexer::internal_error"),
            )?),
            LexerError::LookaheadExceedsMaximum => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message("Lookahead exceeds maximum".to_string())
                    .with_code("parol_runtime::lexer::lookahead_exceeds_maximum"),
            )?),
            LexerError::LookaheadExceedsTokenBufferLength => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message("Lookahead exceeds token buffer length".to_string())
                    .with_code("parol_runtime::lexer::lookahead_exceeds_token_buffer_length"),
            )?),
            LexerError::ScannerStackEmptyError => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message("Tried to pop from empty scanner stack".to_string())
                    .with_code("parol_runtime::lexer::pop_from_empty_scanner_stack")
                    .with_notes(vec![
                        "Check balance of %push and %pop directives in your grammar".to_string(),
                    ]),
            )?),
            LexerError::RecoveryError(e) => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message(format!("Lexer recovery error: {e}"))
                    .with_code("parol_runtime::lexer::recovery"),
            )?),
        }
    }

    fn report_parser_error(
        err: &ParserError,
        writer: &StandardStream,
        config: &term::Config,
        files: &FileDb<String, &str>,
    ) -> anyhow::Result<()> {
        match err {
            ParserError::TreeError { source } => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message(format!("Error from syntree crate: {}", source))
                    .with_code("parol_runtime::parser::syntree_error")
                    .with_notes(vec!["Internal error".to_string()]),
            )?),
            ParserError::DataError(e) => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message(format!("Data error: {e}"))
                    .with_code("parol_runtime::lexer::internal_error")
                    .with_notes(vec!["Error in generated source".to_string()]),
            )?),
            ParserError::PredictionError { cause } => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::error()
                    .with_message("Error in input")
                    .with_code("parol_runtime::lookahead::production_prediction_error")
                    .with_notes(vec![cause.to_string()]),
            )?),
            ParserError::SyntaxErrors { entries } => {
                entries.iter().try_for_each(
                    |SyntaxError {
                         cause,
                         error_location,
                         unexpected_tokens,
                         expected_tokens,
                         source,
                         ..
                     }|
                     -> anyhow::Result<()> {
                        if let Some(source) = source {
                            match source.as_ref() {
                                ParolError::LexerError(x) => {
                                    Self::report_lexer_error(x, writer, config, files)?
                                }
                                ParolError::ParserError(x) => {
                                    Self::report_parser_error(x, writer, config, files)?
                                }
                                _ => (),
                            }
                        }

                        let (range, file_id): (Range<usize>, usize) =
                            if unexpected_tokens.is_empty() {
                                let location: Location = (&**error_location).into();
                                location.range_file(files)
                            } else {
                                let token = &unexpected_tokens[0].token;
                                let range = token.into();
                                let file_id = files
                                    .get_id(&token.file_name.display().to_string())
                                    .unwrap();
                                (range, file_id)
                            };

                        let unexpected_tokens_labels =
                            unexpected_tokens.iter().fold(vec![], |mut acc, un| {
                                acc.push(
                                    Label::secondary(
                                        file_id,
                                        Into::<Range<usize>>::into(&un.token),
                                    )
                                    .with_message(un.token_type.clone()),
                                );
                                acc
                            });
                        Ok(term::emit(
                            &mut writer.lock(),
                            &config,
                            files,
                            &Diagnostic::error()
                                .with_message("Syntax error")
                                .with_code("parol_runtime::parser::syntax_error")
                                .with_labels(vec![
                                    Label::primary(file_id, range).with_message("Found")
                                ])
                                .with_labels(unexpected_tokens_labels)
                                .with_notes(vec![
                                    format!("Expecting {}", expected_tokens),
                                    cause.to_string(),
                                ]),
                        )?)
                    },
                )?;
                Ok(term::emit(
                    &mut writer.lock(),
                    &config,
                    files,
                    &Diagnostic::error()
                        .with_message(format!("{} syntax error(s) found", entries.len())),
                )?)
            }
            ParserError::UnprocessedInput { last_token, .. } => {
                let un_span: Span = (Into::<Range<usize>>::into(&**last_token)).into();
                let file_id = files
                    .get_id(&last_token.file_name.display().to_string())
                    .unwrap();
                Ok(term::emit(
                    &mut writer.lock(),
                    &config,
                    files,
                    &Diagnostic::error()
                        .with_message("Unprocessed input is left after parsing has finished")
                        .with_code("parol_runtime::parser::unprocessed_input")
                        .with_labels(vec![
                            Label::primary(file_id, un_span).with_message("Unprocessed")
                        ])
                        .with_notes(vec![
                            "Unprocessed input could be a problem in your grammar.".to_string(),
                        ]),
                )?)
            }
            ParserError::PopOnEmptyScannerStateStack {
                context, source, ..
            } => {
                Self::report_lexer_error(source, writer, config, files)?;

                Ok(term::emit(
                    &mut writer.lock(),
                    &config,
                    files,
                    &Diagnostic::error()
                        .with_message(format!("{context}Tried to pop from an empty scanner stack"))
                        .with_code("parol_runtime::parser::pop_on_empty_scanner_stack"),
                )?)
            }
            ParserError::InternalError(e) => Ok(term::emit(
                &mut writer.lock(),
                &config,
                files,
                &Diagnostic::bug()
                    .with_message(format!("Internal parser error: {e}"))
                    .with_code("parol_runtime::parser::internal_error")
                    .with_notes(vec!["This may be a bug. Please report it!".to_string()]),
            )?),
        }
    }
}

/// FileDb
#[derive(Clone, Debug)]
pub struct FileDb<Name, Source> {
    files: Vec<SimpleFile<Name, Source>>,
    ids: HashMap<Name, usize>,
}

impl<Name, Source> FileDb<Name, Source>
where
    Name: std::fmt::Display + Clone + PartialEq + Eq + std::hash::Hash,
    Source: AsRef<str>,
{
    /// Create a new files database.
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            ids: HashMap::new(),
        }
    }

    /// Add a file to the database, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, name: Name, source: Source) -> usize {
        let file_id = self.files.len();
        self.files.push(SimpleFile::new(name.clone(), source));
        self.ids.insert(name, file_id);
        file_id
    }

    /// Get the file corresponding to the given id.
    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<Name, Source>, files::Error> {
        self.files.get(file_id).ok_or(files::Error::FileMissing)
    }

    pub fn get_id(&self, name: &Name) -> Option<usize> {
        self.ids.get(name).copied()
    }
}

impl<'a, Name, Source> Files<'a> for FileDb<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone + PartialEq + Eq + std::hash::Hash,
    Source: 'a + AsRef<str>,
{
    type FileId = usize;
    type Name = Name;
    type Source = &'a str;

    fn name(&self, file_id: usize) -> Result<Name, files::Error> {
        Ok(self.get(file_id)?.name().clone())
    }

    fn source(&self, file_id: usize) -> Result<&str, files::Error> {
        Ok(self.get(file_id)?.source().as_ref())
    }

    fn line_index(&self, file_id: usize, byte_index: usize) -> Result<usize, files::Error> {
        self.get(file_id)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: usize, line_index: usize) -> Result<Range<usize>, files::Error> {
        self.get(file_id)?.line_range((), line_index)
    }
}
