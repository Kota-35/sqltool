use std::path::PathBuf;

use clap::Parser;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(
    author,
    name = "sqltool",
    about = "SqlTool: sql formatter written in Rust"
)]
#[command(version)]
#[command(styles = STYLES)]
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
