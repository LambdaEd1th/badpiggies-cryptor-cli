use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Builder;
use log::{LevelFilter, debug, info};
use std::fs;

// Import the decoupled logic from lib.rs
use badpiggies_cryptor_cli::{
    cli::{Cli, Commands, CryptoArgs, GenerateArgs},
    process_data,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logger(&cli);

    debug!("Program started with args: {:?}", cli);

    match cli.command {
        Commands::Encrypt(args) => run_crypto_task(args, true)?,
        Commands::Decrypt(args) => run_crypto_task(args, false)?,
        Commands::Generate(args) => generate_sample(args)?,
    }

    debug!("Program finished successfully");
    Ok(())
}

/// Handles the File I/O for encryption/decryption tasks.
fn run_crypto_task(args: CryptoArgs, is_encrypt: bool) -> Result<()> {
    // 1. I/O Layer: Read input
    debug!("Reading input file: {:?}", args.input_file);
    let data = fs::read(&args.input_file)
        .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

    debug!("Processing data (size: {} bytes)", data.len());

    // 2. Logic Layer: Call the pure function in lib.rs
    // This is now decoupled from the file system.
    let result_data = process_data(&args.file_type, is_encrypt, &data)?;

    // 3. I/O Layer: Write output
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

/// Generates a sample XML file for testing or new saves.
fn generate_sample(args: GenerateArgs) -> Result<()> {
    let output_path = args.get_file();

    // Embed the sample file into the binary
    const SAMPLE_XML: &[u8] = include_bytes!("../tests/sample.xml");

    debug!("Generating sample file at: {:?}", output_path);
    fs::write(&output_path, SAMPLE_XML)
        .with_context(|| format!("Failed to generate sample file: {:?}", output_path))?;

    info!("✅ Sample file generated: {:?}", output_path);
    Ok(())
}

/// Initializes the logging system based on CLI flags or environment variables.
fn init_logger(cli: &Cli) {
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
}
