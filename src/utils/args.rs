use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Suppress all output except errors
    #[arg(global = true, short, long, action = ArgAction::SetTrue, conflicts_with_all = [&"verbose", &"debug"])]
    pub quiet: bool,

    /// Enable verbose logging
    #[arg(global = true, short, long, action = ArgAction::SetTrue, conflicts_with = "debug")]
    pub verbose: bool,

    /// Enable debug logging
    #[arg(global = true, long, action = ArgAction::SetTrue)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a new repository.
    Init {
        /// The name of the repository to create.
        #[arg(default_value = ".")]
        name: String,

        /// The directory to create the repository in.
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
    /// Adds a package to the repository.
    Add {
        /// The path to the package file to add.
        package_path: PathBuf,
    },
    /// Removes a package from the repository.
    Remove {
        /// The name of the package to remove.
        name: String,
        /// The version of the package to remove.
        version: String,
    },
    /// Lists all packages in the repository.
    List,

    /// Builds the package index from the packages directory.
    Build,
}
