use clap::{Arg, Command};
use std::io::{self, Read};
use std::fs;

mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("rustots")
        .about("Analisador Léxico e Sintático para TypeScript")
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Ler da entrada padrão (stdin)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .help("Arquivo de entrada (.ts)")
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::new("only-malformed")
                .long("only-malformed")
                .help("Mostrar apenas tokens com problemas (malformed)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input = if matches.get_flag("stdin") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        fs::read_to_string(file_path)?
        } else {
            eprintln!("Erro: Nenhuma entrada fornecida. Use --stdin ou informe um caminho de arquivo.");
            std::process::exit(1);
        };

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens.clone());
    let mut parse_error: Option<String> = None;
    let ast = match parser.parse() {
        Ok(program) => Some(program),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            eprintln!("Erro de Análise: {}", error_msg);
            parse_error = Some(error_msg);
            None
        }
    };

    let mut result_tokens = tokens;
    if matches.get_flag("only-malformed") {
        result_tokens.retain(|t| t.malformed.is_some());
    }

    let result = serde_json::json!({
        "tokens": result_tokens,
        "ast": ast,
        "error": parse_error
    });
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
