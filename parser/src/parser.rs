use crate::sdc::{Sdc, SdcError};
use crate::sdc_grammar::SdcGrammar;
use crate::sdc_parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    #[allow(clippy::result_large_err)]
    pub fn parse<T: AsRef<Path>>(input: &str, file: &T) -> Result<Sdc, SdcError> {
        let mut grammar = SdcGrammar::new();
        parse(input, file, &mut grammar)?;
        grammar.sdc.unwrap()
    }
}
