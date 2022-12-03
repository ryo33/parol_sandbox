use parol::build::Builder;

fn main() {
    // CLI equivalent is:
    // parol -f ./parol_sandbox.par -e ./parol_sandbox-exp.par -p ./src/parol_sandbox_parser.rs -a ./src/parol_sandbox_grammar_trait.rs -t ParolSandboxGrammar -m parol_sandbox_grammar -g
    Builder::with_explicit_output_dir("src")
        .grammar_file("parol_sandbox.par")
        .expanded_grammar_output_file("../parol_sandbox-exp.par")
        .parser_output_file("parol_sandbox_parser.rs")
        .actions_output_file("parol_sandbox_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("ParolSandboxGrammar")
        .user_trait_module_name("parol_sandbox_grammar")
        .generate_parser()
        .unwrap();
}
