use parser_of_logic_expressions::{LogicParser, Rule, make_parantecies};
use pest::Parser;
use pest::iterators::Pair;

fn collect_atoms(pair: Pair<Rule>, out: &mut Vec<String>) {
    if pair.as_rule() == Rule::atom {
        out.push(pair.as_str().to_string());
        return;
    }
    for pair in pair.into_inner() {
        collect_atoms(pair, out);
    }
}

#[test]
fn atom1() {
    let mut pairs = LogicParser::parse(Rule::atom, "A1_b").expect("parse atom");
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::atom);
    assert_eq!(pair.as_str(), "A1_b");
}

#[test]
fn atom2() {
    let mut pairs = LogicParser::parse(Rule::atom, "true").expect("parse atom true");
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_str(), "true");
}

#[test]
fn primary1() {
    let mut pairs = LogicParser::parse(Rule::primary, "(a)").expect("parse primary");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a"]);
}

#[test]
fn primary2() {
    let mut pairs = LogicParser::parse(Rule::primary, "[true]").expect("parse primary bracket");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["true"]);
}

#[test]
fn primary3() {
    let mut pairs = LogicParser::parse(Rule::primary, "{False}").expect("parse primary bracket");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["False"]);
}

#[test]
fn negation() {
    let mut pairs = LogicParser::parse(Rule::negation, "!!true").expect("parse negation");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["true"]);
}

#[test]
fn and() {
    let mut pairs = LogicParser::parse(Rule::and, "a & b & c").expect("parse and");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a", "b", "c"]);
}

#[test]
fn xor() {
    let mut pairs = LogicParser::parse(Rule::xor, "a ^ b ^ c").expect("parse xor");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a", "b", "c"]);
}

#[test]
fn or() {
    let mut pairs = LogicParser::parse(Rule::or, "a | b | c").expect("parse or");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a", "b", "c"]);
}

#[test]
fn implication() {
    let mut pairs =
        LogicParser::parse(Rule::implication, "a -> b -> c").expect("parse implication");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a", "b", "c"]);
}

#[test]
fn expression() {
    let mut pairs =
        LogicParser::parse(Rule::expression, "a <-> b <-> c").expect("parse expression");
    let pair = pairs.next().unwrap();
    let mut atoms: Vec<String> = Vec::new();
    collect_atoms(pair, &mut atoms);
    assert_eq!(atoms, vec!["a", "b", "c"]);
}

#[test]
fn file() {
    let src = "a & b\ntrue\n";
    let mut pairs = LogicParser::parse(Rule::file, src).expect("parse file");
    let pair = pairs.next().unwrap();
    let mut collected: Vec<Vec<String>> = Vec::new();
    for pair in pair.into_inner() {
        let mut atoms: Vec<String> = Vec::new();
        collect_atoms(pair, &mut atoms);
        if !atoms.is_empty() {
            collected.push(atoms);
        }
    }
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0], vec!["a", "b"]);
    assert_eq!(collected[1], vec!["true"]);
}

#[test]
fn file_with_comments() {
    let src2 = "a\n// comment line\nb\n";
    let mut pairs = LogicParser::parse(Rule::file, src2).expect("parse file comments");
    let pair = pairs.next().unwrap();
    let mut collected: Vec<Vec<String>> = Vec::new();
    for pair in pair.into_inner() {
        let mut atoms: Vec<String> = Vec::new();
        collect_atoms(pair, &mut atoms);
        if !atoms.is_empty() {
            collected.push(atoms);
        }
    }
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0], vec!["a"]);
    assert_eq!(collected[1], vec!["b"]);
}

#[test]
fn make_parantecies_three_atoms() {
    let mut pairs = LogicParser::parse(Rule::expression, "a & b | c").expect("parse expression");
    let pair = pairs.next().unwrap();
    let out = make_parantecies(pair);
    assert_eq!(out, "((a & b) | c)");
}
