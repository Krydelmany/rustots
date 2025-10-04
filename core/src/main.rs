use clap::{Arg, Command};
use std::io::{self, Read};
use std::fs;

mod lexer;

use lexer::Lexer;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("rustots")
        .about("Analisador Léxico para TypeScript")
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
            Arg::new("filter")
                .long("filter")
                .help("Filtrar tipos de token (ex: keyword,identifier)")
                .value_name("TYPES"),
        )
        .arg(
            Arg::new("no-whitespace")
                .long("no-whitespace")
                .help("Omitir tokens de whitespace e newline")
                .action(clap::ArgAction::SetTrue),
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
    let mut tokens = lexer.tokenize();

    // Filtrar tokens se necessário
    if matches.get_flag("no-whitespace") {
        tokens.retain(|t| !matches!(t.token_type, lexer::TokenType::Whitespace | lexer::TokenType::Newline));
    }

    if matches.get_flag("only-malformed") {
        tokens.retain(|t| t.malformed.is_some());
    }

    if let Some(filter_types) = matches.get_one::<String>("filter") {
        let types: Vec<&str> = filter_types.split(',').map(|s| s.trim()).collect();
        tokens.retain(|t| {
            let type_str = format!("{:?}", t.token_type).to_lowercase();
            types.iter().any(|filter| type_str.contains(&filter.to_lowercase()))
        });
    }

    let result = serde_json::json!({
        "tokens": tokens
    });
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
