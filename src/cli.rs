use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(
    help_template = "{before-help}{about} by @{author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
pub struct Cli {
    /// What crypto mode to run the program in
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt mode
    Encrypt(CryptoArgs),

    /// Decrypt mode
    Decrypt(CryptoArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct CryptoArgs {
    /// What file type to run the program in
    #[arg(value_enum)]
    pub file_type: FileTypes,
    
    /// Input file
    #[arg(value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT_FILE")]
    pub output_file: PathBuf,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum FileTypes {
    Progress,
    Contraption,
}