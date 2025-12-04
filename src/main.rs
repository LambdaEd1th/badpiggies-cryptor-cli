use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

mod cli;
mod crypto;

use cli::{Cli, Commands, FileTypes};
use crypto::Cryptor;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cryptor = Cryptor::new();

    match cli.command {
        Commands::Encrypt(args) => {
            let data = fs::read(&args.input_file)
                .with_context(|| format!("Cannot read input file: {:?}", args.input_file))?;

            let output = match args.file_type {
                FileTypes::Progress => cryptor.encrypt_progress(&data),
                FileTypes::Contraption => cryptor.encrypt_contraption(&data),
            };

            fs::write(&args.output_file, output)
                .with_context(|| format!("Cannot write output file: {:?}", args.output_file))?;

            println!("Encryption complete: {:?}", args.output_file);
        }
        Commands::Decrypt(args) => {
            let data = fs::read(&args.input_file)
                .with_context(|| format!("Cannot read input file: {:?}", args.input_file))?;

            let output = match args.file_type {
                FileTypes::Progress => cryptor.decrypt_progress(&data)?,
                FileTypes::Contraption => cryptor.decrypt_contraption(&data)?,
            };

            fs::write(&args.output_file, output)
                .with_context(|| format!("Cannot write output file: {:?}", args.output_file))?;

            println!("Decrypt complete: {:?}", args.output_file);
        }
        Commands::Generate(args) => {
            let output_path = args.get_file();

            const EXAMPLE_XML: &[u8] = include_bytes!("../tests/example.xml");

            fs::write(&output_path, EXAMPLE_XML)
                .with_context(|| format!("Unable to generate sample file: {:?}", output_path))?;

            println!("Sample file has been generated: {:?}", output_path);
        }
    }

    Ok(())
}
