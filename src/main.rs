use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

mod crypto;
use crypto::Cryptor;

mod cli;
use cli::{Cli, CryptoModes, FileTypes};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut input_file = File::open(cli.input_file)?;
    let mut input_buffer: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_buffer)?;
    let output_buffer: Vec<u8>;
    let cryptor = Cryptor::new(&input_buffer);
    match cli.file_type {
        FileTypes::Progress => {
            match cli.crypto_mode {
                CryptoModes::Encrypt => {
                    output_buffer = cryptor.encrypt_progress();
                }
                CryptoModes::Decrypt => {
                    output_buffer = cryptor.decrypt_progress()?;
                }
            }
        }
        FileTypes::Contraption => {
            match cli.crypto_mode {
                CryptoModes::Encrypt => {
                    output_buffer = cryptor.encrypt_contraption();
                }
                CryptoModes::Decrypt => {
                    output_buffer = cryptor.decrypt_contraption()?;
                }
            }
        }
    }
    let mut output_file = File::create(cli.output_file)?;
    output_file.write_all(&output_buffer)?;
    Ok(())
}
