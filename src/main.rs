use std::{
    fs::File,
    io::{Read, Write}, path::PathBuf,
};
use clap::Parser;
use crypto::Cryptor;
use resource::Resource;
use cli::{Cli, Commands, FileTypes};

mod crypto;
mod resource;
mod cli;

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
            if let Some(embedded) = Resource::get("Example.xml") {
                write_file(&output_path, &embedded.data)?;
            } else {
                return Err(Error::Resource);
            }
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
    Crypto(#[from] crypto::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Resource not found")]
    Resource,
}