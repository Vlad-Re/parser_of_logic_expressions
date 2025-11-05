use anyhow::{anyhow, Result};
use pest::Parser;
use parser_of_logic_expressions::{LogicParser, Rule};


#[test]
fn test_parse_simple_atom() -> Result<()> {
    let input = "a";

    let pair = LogicParser::parse(Rule::expression, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "a");

    Ok(())
}
