use anyhow;
use pest::Parser;

use parser_of_logic_expressions::{LogicParser, Rule};

fn main() -> anyhow::Result< () > {
    let got = LogicParser::parse(Rule::expression, "Atom")?;
    println!("{:#?}", got);

    Ok(())
}
