use clap::{Parser, Subcommand};
use anyhow::Result;
mod lexer;

/// RusCom — C++ compiler prototype in Rust (scaffold)
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile C++ source to object / executable
    Compile {
        /// Input source file
        input: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Dump AST (placeholder)
    AstDump { input: String },
    /// Dump lexical tokens from input
    Lex {
        input: String,
        /// Print only the number of tokens instead of dumping them
        #[arg(long = "count")]
        count: bool,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output } => {
            println!("Compile: input={} output={:?}", input, output);
        }
        Commands::AstDump { input } => {
            println!("AST dump: input={}", input);
        }
        Commands::Lex { input, count } => {
            let src = std::fs::read_to_string(&input)?;
            let mut lexer = lexer::Lexer::new(&src);
            if count {
                let mut n = 0usize;
                while let Some(tok) = lexer.next() {
                    match tok {
                        Ok(t) => {
                            if t == lexer::token::Token::Eof { break; }
                            n += 1;
                        }
                        Err(e) => { eprintln!("Lex error: {}", e); break; }
                    }
                }
                println!("{}", n);
            } else {
                while let Some(tok) = lexer.next() {
                    match tok {
                        Ok(t) => println!("{:?}", t),
                        Err(e) => { eprintln!("Lex error: {}", e); break; }
                    }
                }
            }
        }
    }

    Ok(())
}
