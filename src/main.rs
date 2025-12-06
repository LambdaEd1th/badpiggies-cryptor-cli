use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

mod cli;
mod crypto;

use cli::{Cli, Commands, CryptoArgs, FileTypes, GenerateArgs};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(args) => process_crypto(args, true)?,
        Commands::Decrypt(args) => process_crypto(args, false)?,
        Commands::Generate(args) => generate_sample(args)?,
    }

    Ok(())
}

fn process_crypto(args: CryptoArgs, is_encrypt: bool) -> Result<()> {
    let data = fs::read(&args.input_file)
        .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

    let result_data = match (args.file_type, is_encrypt) {
        (FileTypes::Progress, true) => crypto::encrypt_progress(&data),
        (FileTypes::Progress, false) => crypto::decrypt_progress(&data)?,
        (FileTypes::Contraption, true) => crypto::encrypt_contraption(&data),
        (FileTypes::Contraption, false) => crypto::decrypt_contraption(&data)?,
    };

    fs::write(&args.output_file, result_data)
        .with_context(|| format!("Failed to write output file: {:?}", args.output_file))?;

    let action = if is_encrypt {
        "Encryption"
    } else {
        "Decryption"
    };
    println!("✅ {} successful: {:?}", action, args.output_file);

    Ok(())
}

fn generate_sample(args: GenerateArgs) -> Result<()> {
    let output_path = args.get_file();

    const SAMPLE_XML: &[u8] = include_bytes!("../tests/sample.xml");

    fs::write(&output_path, SAMPLE_XML)
        .with_context(|| format!("Failed to generate sample file: {:?}", output_path))?;

    println!("✅ Sample file generated: {:?}", output_path);
    Ok(())
}
