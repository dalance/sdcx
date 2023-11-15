use crate::parser::sdc_grammar::SdcGrammar;
use crate::parser::sdc_parser::parse;
use crate::sdc::{Sdc, SdcError};
use std::path::Path;

pub mod generated;
pub mod sdc_grammar;
pub mod sdc_grammar_trait;
pub mod sdc_parser;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    #[allow(clippy::result_large_err)]
    pub fn parse<T: AsRef<Path>>(input: &str, file: &T) -> Result<Sdc, SdcError> {
        let mut grammar = SdcGrammar::new();
        parse(input, file, &mut grammar)?;
        let sdc = grammar.sdc.unwrap()?;
        sdc.validate()?;
        Ok(sdc)
    }
}
