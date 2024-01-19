use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub ebnf); // synthesized by LALRPOP

#[test]
fn ebnf_test() {
    assert!(ebnf::NonTerminalParser::new().parse("testing =").is_ok());
}

fn main() {
    println!("Hello, World.\n");
}

