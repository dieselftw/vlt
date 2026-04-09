mod commands;
mod models;
mod utils;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vlt", version, about = "Manage .env files across environments")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Scan(ScanArgs),
    Generate,
    Create(CreateArgs),
    Use(CreateArgs),
    Status,
    Diff(DiffArgs),
    Sync(SyncArgs),
    Validate,
}

#[derive(Args, Debug)]
struct ScanArgs {
    #[arg(long, help = "Add missing variables to .vlt/env.rules")]
    apply: bool,
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(help = "Environment name")]
    env_name: String,
}

#[derive(Args, Debug)]
struct DiffArgs {
    #[arg(help = "First environment name")]
    env1: String,
    #[arg(help = "Second environment name")]
    env2: String,
}

#[derive(Args, Debug)]
struct SyncArgs {
    #[arg(help = "Source environment name")]
    source: String,
    #[arg(help = "Target environment name")]
    target: String,
}

fn main() {
    if let Err(error) = try_main() {
        utils::output::print_error(&error);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(),
        Commands::Scan(args) => commands::scan::run(args.apply),
        Commands::Generate => commands::generate::run(),
        Commands::Create(args) => commands::create::run(&args.env_name),
        Commands::Use(args) => commands::use_env::run(&args.env_name),
        Commands::Status => commands::status::run(),
        Commands::Diff(args) => commands::diff::run(&args.env1, &args.env2),
        Commands::Sync(args) => commands::sync::run(&args.source, &args.target),
        Commands::Validate => commands::validate::run(),
    }
}
