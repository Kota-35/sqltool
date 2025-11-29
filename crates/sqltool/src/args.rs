use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author,
    name = "sqltool",
    about = "SqlTool: sql formatter written in Rust"
)]
#[command(version)]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Format(FormatCommand),
}

#[derive(Debug, Clone, clap::Parser)]
pub struct FormatCommand {
    /// List of files or directories to format [default .]
    #[clap(help = "List of files or directories to format [default .]")]
    pub files: Vec<PathBuf>,
}
