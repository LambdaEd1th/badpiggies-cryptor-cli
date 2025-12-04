use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

mod cli;
mod crypto;

use cli::{Cli, Commands, FileTypes};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(args) => {
            let data = fs::read(&args.input_file)
                .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

            let output = match args.file_type {
                FileTypes::Progress => crypto::encrypt_progress(&data),
                FileTypes::Contraption => crypto::encrypt_contraption(&data),
            };

            fs::write(&args.output_file, output)
                .with_context(|| format!("Failed to write output file: {:?}", args.output_file))?;

            println!("✅ Encryption successful: {:?}", args.output_file);
        }
        Commands::Decrypt(args) => {
            let data = fs::read(&args.input_file)
                .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

            let output = match args.file_type {
                FileTypes::Progress => crypto::decrypt_progress(&data)?,
                FileTypes::Contraption => crypto::decrypt_contraption(&data)?,
            };

            fs::write(&args.output_file, output)
                .with_context(|| format!("Failed to write output file: {:?}", args.output_file))?;

            println!("✅ Decryption successful: {:?}", args.output_file);
        }
        Commands::Generate(args) => {
            let output_path = args.get_file();

            const EXAMPLE_XML: &[u8] = include_bytes!("../tests/example.xml");

            fs::write(&output_path, EXAMPLE_XML)
                .with_context(|| format!("Failed to generate example file: {:?}", output_path))?;

            println!("✅ Example file generated: {:?}", output_path);
        }
    }

    Ok(())
}
