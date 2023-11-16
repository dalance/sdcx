use codespan_reporting::files::{self, Files, SimpleFile};
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

/// FileDb
#[derive(Clone, Debug)]
pub struct FileDb<Name, Source> {
    files: Vec<SimpleFile<Name, Source>>,
    ids: HashMap<Name, usize>,
}

impl<Name, Source> Default for FileDb<Name, Source> {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            ids: HashMap::new(),
        }
    }
}

impl<Name, Source> FileDb<Name, Source>
where
    Name: std::fmt::Display + Clone + PartialEq + Eq + std::hash::Hash,
    Source: AsRef<str>,
{
    /// Create a new files database.
    pub fn new() -> Self {
        Self::default()
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
    pub(crate) fn from_to(from: &Location, to: &Location) -> Location {
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

    pub(crate) fn range_file(&self, files: &FileDb<String, &str>) -> (Range<usize>, usize) {
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
