use anyhow::{Context, Result, anyhow};
use pest::Parser;
use pest::iterators::Pair;
use std::env;
use std::fs;

use parser_of_logic_expressions::{LogicParser, Rule, make_parantecies};

fn print_pair(pair: Pair<Rule>, level: usize) {
    let prefix = "  ".repeat(level);
    println!("{prefix}{:?}: '{}'", pair.as_rule(), pair.as_str());
    for inner in pair.into_inner() {
        print_pair(inner, level + 1);
    }
}

fn cmd_help() {
    println!("Allowed commands:");
    println!("parse <FILE> - Parse a file containing one or more expressions (newline-separated)");
    println!("help - Show this help message");
    println!("credits - Show credits and license info");
}

fn cmd_credits() {
    println!("parser_of_logic_expressions - Propositional logic parser");
    println!("Author: Vlad-Re");
    println!("License: MIT");
}

fn cmd_parse(path: &str) -> Result<()> {
    let content = fs::read_to_string(path).with_context(|| format!("reading file '{}'", path))?;
    let pairs = LogicParser::parse(Rule::file, &content)
        .with_context(|| format!("parsing file '{}'", path))?;

    println!("Parse tree for '{path}':");
    for p in pairs {
        print_pair(p, 0);
    }

    Ok(())
}

fn cmd_make_parantecies(in_path: &str, out_path: &str) -> Result<()> {
    let content =
        fs::read_to_string(in_path).with_context(|| format!("reading file '{}'", in_path))?;
    let pairs = LogicParser::parse(Rule::file, &content)
        .with_context(|| format!("parsing file '{}'", in_path))?;

    let mut results: Vec<String> = Vec::new();
    for p in pairs {
        for child in p.into_inner() {
            if child.as_rule() == Rule::expression {
                let s = make_parantecies(child);
                results.push(s);
            }
        }
    }

    let mut out_text = String::new();
    let mut i = 0_usize;
    while i < results.len() {
        out_text.push_str(&results[i]);
        out_text.push('\n');
        i += 1_usize;
    }

    fs::write(out_path, out_text).with_context(|| format!("writing file '{}'", out_path))?;
    println!("Wrote {} lines to '{}'", results.len(), out_path);
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let cmd_arg = args.next();
    if cmd_arg.is_none() {
        cmd_help();
        return Ok(());
    }

    let cmd = cmd_arg.unwrap();
    if cmd == "help" {
        cmd_help();
        return Ok(());
    }

    if cmd == "credits" {
        cmd_credits();
        return Ok(());
    }

    if cmd == "parse" {
        let path_arg = args.next();
        if let Some(path) = path_arg {
            return cmd_parse(&path);
        }
        return Err(anyhow!("'parse' command requires a file path"));
    }

    if cmd == "make_parantecies" {
        let in_arg = args.next();
        let out_arg = args.next();
        if let (Some(input), Some(output)) = (in_arg, out_arg) {
            return cmd_make_parantecies(&input, &output);
        }
        return Err(anyhow!(
            "'make_parantecies' requires input and output file paths"
        ));
    }

    println!("Unknown command: {cmd}");
    cmd_help();
    Ok(())
}
