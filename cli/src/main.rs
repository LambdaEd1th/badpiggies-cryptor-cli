use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info};
use std::fs;

// Import core library
use badpiggies_cryptor_core::{
    constants::TEMPLATE_XML, mode::CryptoMode, process_data, Categories as CoreCategories,
};

// Import local cli module
mod cli;
use cli::{Cli, Commands, CryptoArgs, GenerateArgs};

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.init_logger();

    debug!("Program started with args: {:?}", cli);

    match cli.command {
        Commands::Encrypt(args) => run_crypto_task(args, CryptoMode::Encrypt)?,
        Commands::Decrypt(args) => run_crypto_task(args, CryptoMode::Decrypt)?,
        Commands::Generate(args) => generate_template(args)?,
    }

    debug!("Program finished successfully");
    Ok(())
}

/// Handles the File I/O for encryption/decryption tasks.
fn run_crypto_task(args: CryptoArgs, mode: CryptoMode) -> Result<()> {
    // 1. I/O Layer: Read input
    debug!("Reading input file: {:?}", args.input);
    let data = fs::read(&args.input)
        .with_context(|| format!("Failed to read input file: {:?}", args.input))?;

    debug!("Processing data (size: {} bytes)", data.len());

    // 2. Logic Layer: Call the pure function in lib.rs
    // Map CLI category to Core category
    let core_category: CoreCategories = args.category.into();

    let result_data = process_data(&core_category, mode, &data)?;

    // 3. I/O Layer: Write output
    let output_path = args.get_output_file(mode);

    debug!("Writing output to: {:?}", output_path);
    fs::write(&output_path, result_data)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    let action = match mode {
        CryptoMode::Encrypt => "Encryption",
        CryptoMode::Decrypt => "Decryption",
    };
    info!("✅ {} successful: {:?}", action, output_path);

    Ok(())
}

/// Generates a template XML file for testing or new saves.
fn generate_template(args: GenerateArgs) -> Result<()> {
    let output_path = args.output;

    debug!("Generating template file at: {:?}", output_path);
    fs::write(&output_path, TEMPLATE_XML)
        .with_context(|| format!("Failed to generate template file: {:?}", output_path))?;

    info!("✅ Template file generated: {:?}", output_path);
    Ok(())
}
