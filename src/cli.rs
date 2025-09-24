use std::num::ParseIntError;
use clap::{
    Parser,
    Subcommand,
    ValueEnum,
    Args,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum CliError {
    #[error("Failed to parse integer argument")]
    Io(#[from] ParseIntError),
    #[error("Argument must be greater or equal to 1")]
    BelowOne
}

fn none_zeor_parser(s: &str) -> Result<usize, CliError> {
    let u = s.parse()?;
    if u == 0 {
        return Err(CliError::BelowOne);
    }
    Ok(u)
}

/// Package manager for vim
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: Command,
}
#[derive(Args, Debug)]
pub struct List {
    /// List start packages
    #[arg(short, long, conflicts_with="opt")]
    pub start: bool,
    /// List optional packages
    #[arg(short, long, conflicts_with="start")]
    pub opt: bool,
    ///List detached(untracked) packages
    #[arg(short, long)]
    pub detached: bool,
    #[arg(short, long, value_name="CATEGORY")]
    pub category: Option<String>,
}

#[derive(Args, Debug)]
pub struct Install  {
        /// Install plugins as opt(ional)
        #[arg(short, long)]
        pub opt: bool,
        /// Install package under provided category
        #[arg(short, long, value_name="CATEGORY", default_value="default")]
        pub category: String,
        /// Install local plugins
        #[arg(short, long)]
        pub local: bool,
        /// Command for loading the plugins
        #[arg(long, value_name="LOAD_CMD")]
        pub on: Option<String>,
        /// Load this plugins for specific types
        #[arg(long = "for", value_name="TYPES")]
        pub for_: Option<String>,
        /// Load this plugins for specific types
        #[arg(long, value_name="BUILD_CMD")]
        pub build: Option<String>,
        /// Load this plugins for specific types
        #[arg(long, value_name="BRANCH")]
        pub branch: Option<String>,
        /// Installing packages concurrently
        #[arg(
            long,
            short = 'j',
            value_name="THREADS",
            value_parser=none_zeor_parser,
            )]
        pub threads: Option<usize>,
        pub package: String,
    }

#[derive(Args, Debug)]
pub struct Uninstall {
        /// remove all package related configuration as well
        #[arg(short, long)]
        pub all: bool,
        pub package: Vec<String>,
    }

#[derive(Args, Debug)]
pub struct Config{
        /// Delete package configuration file
        #[arg(short, long)]
        pub delete: bool,
        pub package: String,
}

#[derive(Args, Debug)]
pub struct Move{
        /// Make package optional
        #[arg(short, long, conflicts_with="category")]
        pub opt: bool,
        /// Package to move
        pub package: String,
        /// Category to move the package to
        #[arg(conflicts_with="opt", default_value="default")]
        pub category: String,
}

#[derive(Args, Debug)]
pub struct Update{
        /// Skip packages
        #[arg(short, long)]
        pub skip: Vec<String>,
        /// Regenerate the '_pack' file (combine all package configurations)
        #[arg(short, long)]
        pub packfile: bool,
        /// Installing packages concurrently
        #[arg(
            long,
            short = 'j',
            value_name="THREADS",
            value_parser=none_zeor_parser,
            )]
        pub threads: Option<usize>,
        /// Packages to update, default all
        pub package: Vec<String>,
    }
#[derive(Args, Debug)]
pub struct Completions{
        #[arg(value_enum)]
        pub shell: Shell
}
#[derive(Subcommand, Debug)]
pub enum Command {
    /// List installed packages
    List(List),
    /// Install new packages/plugins
    Install(Install),
    /// Uninstall packages/plugins
    Uninstall(Uninstall) ,
    /// Configure/edit the package specific configuration
    Config(Config),
    /// Move a package to a different category or make it optional.
    Move(Move),
    /// Update packages
    Update(Update),
    /// Generate the pack package file
    Generate,
    /// Generates completion scripts for your shell
    Completions(Completions),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    /// Z Shell
    Zsh,
}
