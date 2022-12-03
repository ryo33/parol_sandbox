use crate::parol_sandbox_grammar_trait::{ ParolSandbox, ParolSandboxGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::miette::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our ParolSandbox grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct ParolSandboxGrammar<'t> {
    pub parol_sandbox: Option<ParolSandbox<'t>>,
}

impl ParolSandboxGrammar<'_> {
    pub fn new() -> Self {
        ParolSandboxGrammar::default()
    }
}

impl Display for ParolSandbox<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for ParolSandboxGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.parol_sandbox {
            Some(parol_sandbox) => writeln!(f, "{}", parol_sandbox),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> ParolSandboxGrammarTrait<'t> for ParolSandboxGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'ParolSandbox'
    fn parol_sandbox(&mut self, arg: &ParolSandbox<'t>) -> Result<()> {
        self.parol_sandbox = Some(arg.clone());
        Ok(())
    }
}
