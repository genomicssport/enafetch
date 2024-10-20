use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct EnaArgs {
    /// please provide the path to the folder to be downloaded
    pub project_arg: String
}
