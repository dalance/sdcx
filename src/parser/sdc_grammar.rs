use crate::parser::sdc_grammar_trait::*;
use crate::sdc::{Sdc, SdcError};
use parol_runtime::ParolError;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Default)]
pub struct SdcGrammar<'a> {
    pub ast: Option<Source<'a>>,
    pub sdc: Option<Result<Sdc, SdcError>>,
}

impl SdcGrammar<'_> {
    pub fn new() -> Self {
        SdcGrammar::default()
    }
}

impl Display for Source<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{self:?}")
    }
}

impl Display for SdcGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.ast {
            Some(ast) => writeln!(f, "{ast}"),
            None => writeln!(f, "No parse result"),
        }
    }
}

impl<'a> SdcGrammarTrait<'a> for SdcGrammar<'a> {
    /// Semantic action for non-terminal 'Source'
    fn source(&mut self, arg: &Source<'a>) -> Result<(), ParolError> {
        self.ast = Some(arg.clone());
        self.sdc = Some(arg.try_into());
        Ok(())
    }
}
