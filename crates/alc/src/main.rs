use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "alc", version, about = "Rust agent-linux-control prototype")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print token-efficient agent manifest.
    Manifest {
        /// Emit compact JSON.
        #[arg(long)]
        compact: bool,
    },
    /// Summarize synthetic benchmark samples. Used to verify Rust core math.
    BenchSummary { samples: Vec<f64> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Manifest { compact } => {
            let manifest = alc_core::brief_manifest();
            if compact {
                println!("{}", serde_json::to_string(&manifest)?);
            } else {
                println!("{}", serde_json::to_string_pretty(&manifest)?);
            }
        }
        Command::BenchSummary { samples } => {
            println!(
                "{}",
                serde_json::to_string(&alc_core::benchmark_summary(&samples))?
            );
        }
    }
    Ok(())
}
