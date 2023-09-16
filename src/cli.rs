use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None, help_template = "{before-help}{about} by @{author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}")]
pub struct Cli {
    /// What file type to run the program in
    #[arg(value_enum)]
    pub file_type: FileTypes,

    /// What crypto mode to run the program in
    #[arg(value_enum)]
    pub crypto_mode: CryptoModes,

    /// Input file
    #[arg(value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT_FILE")]
    pub output_file: PathBuf,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CryptoModes {
    Encrypt,
    Decrypt,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum FileTypes {
    Progress,
    Contraption,
}
