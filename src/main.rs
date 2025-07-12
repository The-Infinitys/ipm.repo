use clap::Parser;
use env_logger::Builder;
use ipm_repo::modules::repo::Repository;
use ipm_repo::utils::args::{Cli, Commands};
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

    match cli.command {
        Commands::Init { name, directory } => {
            let repo = Repository::init(name, directory)?;
            log::info!(
                "Repository initialized at {}",
                repo.path.display()
            );
        }
        Commands::Add { package_path } => {
            let repo =
                Repository::load(std::env::current_dir()?)?;
            repo.add_package(package_path)?;
            log::info!("Package added to repository.");
        }
        Commands::Remove { name, version } => {
            let repo =
                Repository::load(std::env::current_dir()?)?;
            repo.remove_package(name, version)?;
            log::info!("Package removed from repository.");
        }
        Commands::List => {
            let repo =
                Repository::load(std::env::current_dir()?)?;
            for package_data in repo.list_packages()? {
                log::info!(
                    "{} - {}",
                    package_data.about.package.name,
                    package_data.about.package.version
                );
            }
        }
        Commands::Build => todo!(),
    }

    Ok(())
}
