use clap::{Parser, Subcommand};
use anyhow::Result;

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
    }

    Ok(())
}
