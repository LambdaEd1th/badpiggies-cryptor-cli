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
    pub output_file: Option<PathBuf>,
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

impl CryptoArgs {
    /// Returns the final output file path. If not provided, generates a default name like 'input_file_decrypted.ext'.
    pub fn get_output_file(&self, is_encrypt: bool) -> PathBuf {
        if let Some(path) = &self.output_file {
            return path.clone();
        }

        let suffix = if is_encrypt {
            "_encrypted"
        } else {
            "_decrypted"
        };

        let input_path = &self.input_file;
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
