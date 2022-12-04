use crate::parol_sandbox_grammar_trait::{Expr, ParolSandboxGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::miette::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our ParolSandbox grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct ParolSandboxGrammar<'t> {
    pub expr: Option<Expr<'t>>,
}

impl ParolSandboxGrammar<'_> {
    pub fn new() -> Self {
        ParolSandboxGrammar::default()
    }
}

impl<'t> ParolSandboxGrammarTrait<'t> for ParolSandboxGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'ParolSandbox'
    fn expr(&mut self, arg: &Expr<'t>) -> Result<()> {
        self.expr = Some(arg.clone());
        Ok(())
    }
}
