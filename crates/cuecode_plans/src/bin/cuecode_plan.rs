use std::path::PathBuf;
use std::process;

use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use cuecode_plans::validate_worktree_or_error;

#[derive(Parser, Debug)]
#[command(name = "cuecode-plan", about = "CueCode plan manifest utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Validate `.cuecode/plans/project.yaml` (or v1 alias) for a worktree.
    Validate {
        /// Project root containing `.cuecode/`.
        #[arg(long, default_value = ".")]
        project_root: PathBuf,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Validate { project_root } => {
            let root = project_root
                .canonicalize()
                .with_context(|| format!("resolving project root {}", project_root.display()))?;
            validate_worktree_or_error(&root)?;
            println!("plan manifest valid: {}", root.display());
        }
    }
    Ok(())
}
