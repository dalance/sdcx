use crate::parser::sdc_grammar::SdcGrammar;
use crate::parser::sdc_parser::parse;
use crate::sdc::{Sdc, SdcError};
use parol_runtime::Report;
use std::path::Path;

pub mod generated;
pub mod sdc_grammar;
pub mod sdc_grammar_trait;
pub mod sdc_parser;

#[derive(Debug)]
pub struct Parser {}

struct Reporter;
impl Report for Reporter {}

impl Parser {
    #[allow(clippy::result_large_err)]
    pub fn parse<T: AsRef<Path>>(input: &str, file: &T) -> Result<Sdc, SdcError> {
        let mut grammar = SdcGrammar::new();
        let ret = parse(input, file, &mut grammar);

        if let Err(err) = ret {
            Reporter::report_error(&err, file)?;
        }

        grammar.sdc.unwrap()
    }
}
