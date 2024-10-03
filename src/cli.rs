use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

const HELP_TEMPLATE: &str = "{before-help}{about} by @{author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}";

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(help_template = HELP_TEMPLATE)]
pub struct Cli {
    /// What mode to run the program in
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt mode
    Encrypt(CryptoArgs),

    /// Decrypt mode
    Decrypt(CryptoArgs),

    /// Generate an example Progress.dat.xml
    Generate(GenerateArgs),
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

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
    /// Output file
    /// (Default: Progress.dat.xml on the current folder)
    #[arg(value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}

impl GenerateArgs {
    pub fn get_file(&self) -> PathBuf {
        match &self.output_file {
            Some(file_name) => file_name.to_path_buf(),
            None => PathBuf::from("./Progress.dat.xml"),
        }
    }
}
