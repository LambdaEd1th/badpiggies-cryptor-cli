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
            let data = read_file(&args.input_file)?;
            let output = match args.file_type {
                FileTypes::Progress => cryptor.encrypt_progress(&data),
                FileTypes::Contraption => cryptor.encrypt_contraption(&data),
            };
            write_file(&args.output_file, &output)?;
        }
        cli::Commands::Decrypt(args) => {
            let data = read_file(&args.input_file)?;
            let output = match args.file_type {
                FileTypes::Progress => cryptor.decrypt_progress(&data)?,
                FileTypes::Contraption => cryptor.decrypt_contraption(&data)?,
            };
            write_file(&args.output_file, &output)?;
        }
        cli::Commands::Generate(args) => {
            let output_path = args.get_file();
            write_file(&output_path, &Resource::get("Example.xml").unwrap().data)?;
        }
    }

    Ok(())
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_file(path: &PathBuf, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CryptoError(#[from] crypto::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
