use clap::{Args, Parser, Subcommand, ValueEnum};
use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;

use crate::mode::CryptoMode;

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

    /// Enable verbose logging (Debug level)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Encrypt mode
    Encrypt(CryptoArgs),
    /// Decrypt mode
    Decrypt(CryptoArgs),
    /// Initialize a sample Progress.dat.xml file
    InitSample(InitSampleArgs),
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct CryptoArgs {
    /// Category of file to encrypt or decrypt
    #[arg(value_enum)]
    pub category: Categories,
    /// Input file to encrypt or decrypt
    #[arg(short, long, value_name = "INPUT_FILE")]
    pub input: PathBuf,
    /// Output file for the encrypted or decrypted data
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output: Option<PathBuf>,
}

/// Enum representing the types of files that can be processed
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Categories {
    Progress,
    Contraption,
}

/// Arguments for the Generate command to create a sample Progress.dat.xml file
#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct InitSampleArgs {
    /// Output file
    #[arg(short, long, default_value = "Progress.dat.xml")]
    pub output: PathBuf,
}

impl Cli {
    /// Initializes the logging system based on CLI flags or environment variables.
    pub fn init_logger(&self) {
        let mut builder = Builder::from_default_env();

        if self.verbose {
            // -v: Show Debug information
            builder.filter_level(LevelFilter::Debug);
        } else if self.quiet {
            // -q: Only show Errors, suppress Info
            builder.filter_level(LevelFilter::Error);
        } else {
            // Default: If RUST_LOG env var is not set, default to Info
            if std::env::var("RUST_LOG").is_err() {
                builder.filter_level(LevelFilter::Info);
            }
        }

        builder.init();
    }
}

impl CryptoArgs {
    /// Returns the final output file path. If not provided, generates a default name like 'input_file_decrypted.ext'.
    pub fn get_output_file(&self, mode: CryptoMode) -> PathBuf {
        if let Some(path) = &self.output {
            return path.clone();
        }

        let suffix = match mode {
            CryptoMode::Encrypt => "_encrypted",
            CryptoMode::Decrypt => "_decrypted",
        };

        let input_path = &self.input;
        let mut output_path = input_path.to_path_buf();

        if let Some(ext) = output_path.extension() {
            let stem = output_path.file_stem().unwrap_or_default();
            let new_file_name = format!(
                "{}{}{}",
                stem.to_string_lossy(),
                suffix,
                &format!(".{}", ext.to_string_lossy())
            );

            output_path.set_file_name(new_file_name);
        } else {
            let original_name = input_path
                .file_name()
                .map(|f| f.to_os_string())
                .unwrap_or_else(|| "output".into());

            let new_file_name = format!("{}{}", original_name.to_string_lossy(), suffix);
            output_path.set_file_name(new_file_name);
        }

        output_path
    }
}
