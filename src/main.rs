use clap::Parser;
use env_logger::Builder;
use ipm_repo::utils::args::Cli;
use log::LevelFilter;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut builder = Builder::from_default_env();
    let log_level = if cli.quiet {
        LevelFilter::Off
    } else if cli.debug {
        LevelFilter::Debug
    } else if cli.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };

    builder.filter(None, log_level).init();

    Ok(())
}
