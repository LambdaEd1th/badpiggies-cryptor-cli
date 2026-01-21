use clap::{Args, Parser, Subcommand, ValueEnum};
use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;

use badpiggies_cryptor_core::mode::CryptoMode;

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
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
    /// Generate a template Progress.dat.xml file
    Generate(GenerateArgs),
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

/// Arguments for the Generate command to create a template Progress.dat.xml file
#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
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

        // Get the filename as a string to manually manipulate string slices.
        // This avoids potential ambiguity with Path::file_stem() in some edge cases.
        let filename_cow = input_path
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default();
        let filename = filename_cow.as_ref();

        // Find the last dot to insert the suffix correctly
        if let Some(idx) = filename.rfind('.') {
            // Split at the last dot: "Progress" | ".xml"
            let (stem, ext) = filename.split_at(idx);
            let new_name = format!("{}{}{}", stem, suffix, ext);
            output_path.set_file_name(new_name);
        } else {
            // No extension found, just append suffix to the end
            let new_name = format!("{}{}", filename, suffix);
            output_path.set_file_name(new_name);
        }

        output_path
    }
}
