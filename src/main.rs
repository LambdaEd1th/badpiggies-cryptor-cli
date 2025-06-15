use clap::Parser;
use cli::{Cli, Commands, FileTypes};
use crypto::Cryptor;
use resource::Resource;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

mod cli;
mod crypto;
mod resource;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cryptor = Cryptor::new();

    match cli.command {
        Commands::Encrypt(args) => {
            let data = read_file_to_buffer(&args.input_file).map_err(|e| {
                eprintln!("Error reading input file: {}", e);
                e
            })?;
            let output = match args.file_type {
                FileTypes::Progress => cryptor.encrypt_progress(&data),
                FileTypes::Contraption => cryptor.encrypt_contraption(&data),
            };
            write_buffer_to_file(&args.output_file, &output).map_err(|e| {
                eprintln!("Error writing output file: {}", e);
                e
            })?;
        }
        cli::Commands::Decrypt(args) => {
            let data = read_file_to_buffer(&args.input_file).map_err(|e| {
                eprintln!("Error reading input file: {}", e);
                e
            })?;
            let output = match args.file_type {
                FileTypes::Progress => cryptor.decrypt_progress(&data).map_err(|e| {
                    eprintln!("Error decrypting Progress file: {}", e);
                    e
                })?,
                FileTypes::Contraption => cryptor.decrypt_contraption(&data).map_err(|e| {
                    eprintln!("Error decrypting Contraption file: {}", e);
                    e
                })?,
            };
            write_buffer_to_file(&args.output_file, &output).map_err(|e| {
                eprintln!("Error writing output file: {}", e);
                e
            })?;
        }
        cli::Commands::Generate(args) => {
            let output_path = args.get_file();
            write_buffer_to_file(&output_path, &Resource::get("Example.xml").unwrap().data)?;
        }
    }

    Ok(())
}

fn read_file_to_buffer(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_buffer_to_file(path: &PathBuf, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
enum Error {
    CryptorError(#[from] crypto::Error),
    IoError(#[from] std::io::Error),
}
