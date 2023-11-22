pub mod generated;
pub mod sdc_grammar;
pub mod sdc_grammar_trait;
pub mod sdc_parser;

use crate::errors::ParseError;
use crate::parser::sdc_grammar::SdcGrammar;
use crate::parser::sdc_parser::parse;
use crate::sdc::Sdc;
use parol_runtime::ParolError;
use std::path::Path;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    #[allow(clippy::result_large_err)]
    pub fn parse<T: AsRef<Path>>(input: &str, file: &T) -> Result<Sdc, ParseError> {
        let mut grammar = SdcGrammar::new();
        match parse(input, file, &mut grammar) {
            Err(ParolError::LexerError(x)) => return Err(ParseError::LexicalError(x)),
            Err(ParolError::ParserError(x)) => return Err(ParseError::SyntaxError(x)),
            _ => (),
        }
        Ok(grammar.sdc.unwrap()?)
    }
}
