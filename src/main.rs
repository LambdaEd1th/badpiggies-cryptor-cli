use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Builder;
use log::{LevelFilter, debug, info};
use std::fs;

mod cli;
mod crypto;

use cli::{Cli, Commands, CryptoArgs, FileTypes, GenerateArgs};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // --- Initialize Logging System ---
    let mut builder = Builder::from_default_env();

    if cli.verbose {
        // -v: Show Debug information
        builder.filter_level(LevelFilter::Debug);
    } else if cli.quiet {
        // -q: Only show Errors, suppress Info
        builder.filter_level(LevelFilter::Error);
    } else {
        // Default: If RUST_LOG env var is not set, default to Info
        if std::env::var("RUST_LOG").is_err() {
            builder.filter_level(LevelFilter::Info);
        }
    }

    builder.init();
    // --------------------------------

    debug!("Program started with args: {:?}", cli);

    match cli.command {
        Commands::Encrypt(args) => process_crypto(args, true)?,
        Commands::Decrypt(args) => process_crypto(args, false)?,
        Commands::Generate(args) => generate_sample(args)?,
    }

    debug!("Program finished successfully");
    Ok(())
}

fn process_crypto(args: CryptoArgs, is_encrypt: bool) -> Result<()> {
    debug!("Reading input file: {:?}", args.input_file);
    let data = fs::read(&args.input_file)
        .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

    debug!("Processing data (size: {} bytes)", data.len());
    let result_data = match (&args.file_type, is_encrypt) {
        (&FileTypes::Progress, true) => crypto::encrypt_progress(&data),
        (&FileTypes::Progress, false) => crypto::decrypt_progress(&data)?,
        (&FileTypes::Contraption, true) => crypto::encrypt_contraption(&data),
        (&FileTypes::Contraption, false) => crypto::decrypt_contraption(&data)?,
    };

    let output_path = args.get_output_file(is_encrypt);

    debug!("Writing output to: {:?}", output_path);
    fs::write(&output_path, result_data)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    let action = if is_encrypt {
        "Encryption"
    } else {
        "Decryption"
    };

    info!("✅ {} successful: {:?}", action, output_path);

    Ok(())
}

fn generate_sample(args: GenerateArgs) -> Result<()> {
    let output_path = args.get_file();

    const SAMPLE_XML: &[u8] = include_bytes!("../tests/sample.xml");

    debug!("Generating sample file at: {:?}", output_path);
    fs::write(&output_path, SAMPLE_XML)
        .with_context(|| format!("Failed to generate sample file: {:?}", output_path))?;

    info!("✅ Sample file generated: {:?}", output_path);
    Ok(())
}
