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
use parol_sandbox_grammar_trait::ParolSandbox;

pub fn parse(input: &str) -> ParolSandbox {
    let mut grammar = parol_sandbox_grammar::ParolSandboxGrammar::new();
    parol_sandbox_parser::parse(input, "dummy", &mut grammar).unwrap();
    grammar.parol_sandbox.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            parse(
                r#"
        "abc"
        "#
            )
            .parol_sandbox_list
            .into_iter()
            .map(|x| match *x.parol_sandbox_list_group {
                parol_sandbox_grammar_trait::ParolSandboxListGroup::ParolSandboxListGroup0(x) =>
                    x.a.text().to_string(),
                parol_sandbox_grammar_trait::ParolSandboxListGroup::ParolSandboxListGroup1(x) =>
                    x.b.text().to_string(),
                parol_sandbox_grammar_trait::ParolSandboxListGroup::ParolSandboxListGroup2(x) =>
                    x.c.text().to_string(),
            })
            .collect::<Vec<_>>(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
