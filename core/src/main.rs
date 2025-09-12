use clap::{Arg, Command};
use std::io::{self, Read};
use std::fs;

mod lexer;
mod parse;
mod types;
mod symbols;
mod collect;
mod diagnostics;

use lexer::Lexer;
use diagnostics::DiagnosticCollector;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("rustots")
        .version("0.1.0")
        .about("TypeScript Static Analyzer")
        .arg(
            Arg::new("lex")
                .long("lex")
                .help("Only perform lexical analysis")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read from stdin")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .help("Input file")
                .value_name("FILE")
                .index(1),
        )
        .get_matches();

    let input = if matches.get_flag("stdin") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        fs::read_to_string(file_path)?
    } else {
        eprintln!("Error: No input provided. Use --stdin or provide a file path.");
        std::process::exit(1);
    };

    let mut diagnostics = DiagnosticCollector::new();
    let mut lexer = Lexer::new(&input, &mut diagnostics);
    let tokens = lexer.tokenize();

    if matches.get_flag("lex") {
        // Only lexical analysis
        let result = serde_json::json!({
            "tokens": tokens,
            "diagnostics": diagnostics.get_diagnostics()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        // Full analysis (placeholder)
        let result = serde_json::json!({
            "tokens": tokens,
            "diagnostics": diagnostics.get_diagnostics()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
