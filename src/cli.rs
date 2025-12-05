// cli.rs
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(
    name = "badpiggies-cryptor-cli",
    author = "ed1th",
    version,
    about = "Bad Piggies user data cryptor",
    long_about = "A command-line tool to encrypt and decrypt Bad Piggies game data files (Progress.dat and .contraption)."
)]
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
    /// Generate an example Progress.dat.xml file
    Generate(GenerateArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct CryptoArgs {
    /// Type of file to encrypt or decrypt
    #[arg(value_enum)]
    pub file_type: FileTypes,
    /// Input file to encrypt or decrypt
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input_file: PathBuf,
    /// Output file for the encrypted or decrypted data
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: PathBuf,
}

/// Enum representing the types of files that can be processed
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum FileTypes {
    Progress,
    Contraption,
}

/// Arguments for the Generate command to create an example Progress.dat.xml file
#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
    /// Output file (Default: Progress.dat.xml on the current folder)
    #[arg(value_name = "OUTPUT_FILE")]
    pub output_file: Option<PathBuf>,
}

impl GenerateArgs {
    /// Returns the output file path, defaulting to "Progress.dat.xml" in the current directory if not specified
    pub fn get_file(&self) -> PathBuf {
        self.output_file
            .clone()
            .unwrap_or_else(|| PathBuf::from("./Progress.dat.xml"))
    }
}
