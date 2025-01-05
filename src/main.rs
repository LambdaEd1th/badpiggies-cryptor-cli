use std::{
    fs::File,
    io::{Read, Write},
};

mod crypto;
use crypto::Cryptor;

mod cli;
use cli::{Cli, Commands, FileTypes};

mod resource;
use resource::Resource;

use clap::Parser;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer = Vec::<u8>::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new();
            let output_buffer;
            match args.file_type {
                FileTypes::Progress => {
                    output_buffer = cryptor.encrypt_progress(&input_file_buffer);
                }
                FileTypes::Contraption => {
                    output_buffer = cryptor.encrypt_contraption(&input_file_buffer);
                }
            }
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
        Commands::Decrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer = Vec::<u8>::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new();
            let output_buffer;
            match args.file_type {
                FileTypes::Progress => {
                    output_buffer = cryptor.decrypt_progress(&input_file_buffer)?;
                }
                FileTypes::Contraption => {
                    output_buffer = cryptor.decrypt_contraption(&input_file_buffer)?;
                }
            }
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
        Commands::Generate(args) => {
            let output_file = args.get_file();
            match Resource::get("Example.xml") {
                Some(embedded_file) => {
                    let mut output_file = File::create(output_file)?;
                    output_file.write_all(&embedded_file.data)?;
                }
                None => return Err(Error::ResourceError),
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    CryptorError(crypto::Error),
    IOError(std::io::Error),
    ResourceError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::CryptorError(err) => write!(f, "CryptorError: {err}"),
            Self::IOError(err) => write!(f, "IOError: {err}"),
            Self::ResourceError => write!(f, "ResourceError"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::CryptorError(err) => Some(err),
            Self::IOError(err) => Some(err),
            Self::ResourceError => None,
        }
    }
}

impl From<crypto::Error> for Error {
    fn from(err: crypto::Error) -> Self {
        Self::CryptorError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}
