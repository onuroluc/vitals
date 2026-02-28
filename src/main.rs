use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use vitals::checks;
use vitals::config;
use vitals::detect;
use vitals::output;
use vitals::platform::Platform;

/// Universal development environment doctor.
///
/// Auto-detects Node, Python, Rust, Go, Ruby, Java, Docker, env files, and
/// more — then checks versions, dependencies, services, ports, and
/// environment variables against project requirements.
#[derive(Parser, Debug)]
#[command(name = "vitals", version, about)]
struct Cli {
    /// Project directory to check
    #[arg(default_value = ".")]
    path: PathBuf,

    /// CI mode: no colors, exit code 1 on failures
    #[arg(long)]
    ci: bool,

    /// Skip check categories (comma-separated: runtime,deps,services,ports,env,commands)
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,

    /// Generate a .vitals.toml template in the current directory
    #[arg(long)]
    init: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // --init: generate template config
    if cli.init {
        let path = cli.path.join(".vitals.toml");
        if path.exists() {
            eprintln!("  .vitals.toml already exists");
            std::process::exit(1);
        }
        std::fs::write(&path, config::template())?;
        println!("  Created .vitals.toml — edit it for your project");
        return Ok(());
    }

    let platform = Platform::detect();
    let config = config::load(&cli.path)?;
    let ctx = detect::scan(&cli.path)?;

    let results = checks::run_all(&ctx, &config, &platform, &cli.skip);
    let (_passed, failed) = output::print_results(&results, cli.ci);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
