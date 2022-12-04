// auto generation needs derive_builder
mod derive_builder {
    pub use parol_runtime::derive_builder::*;
}

extern crate parol_runtime;

mod parol_sandbox_grammar;
pub use parol_sandbox_grammar::ParolSandboxGrammar;

mod parol_sandbox_grammar_trait;
pub use parol_sandbox_grammar_trait::ASTType;

mod parol_sandbox_parser;
use parol_sandbox_grammar_trait::Expr;

pub fn parse(input: &str) -> Expr {
    let mut grammar = parol_sandbox_grammar::ParolSandboxGrammar::new();
    parol_sandbox_parser::parse(input, "dummy", &mut grammar).unwrap();
    grammar.expr.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse(
            r#"
            ! -> abc abc
            "#,
        );
    }
}
