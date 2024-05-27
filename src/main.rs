use std::{
    error::Error,
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer: Vec<u8> = Vec::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new(&input_file_buffer);
            let output_buffer;
            match args.file_type {
                FileTypes::Progress => {
                    output_buffer = cryptor.encrypt_progress();
                }
                FileTypes::Contraption => {
                    output_buffer = cryptor.encrypt_contraption();
                }
            }
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
        Commands::Decrypt(args) => {
            let mut input_file = File::open(args.input_file)?;
            let mut input_file_buffer: Vec<u8> = Vec::new();
            input_file.read_to_end(&mut input_file_buffer)?;
            let cryptor = Cryptor::new(&input_file_buffer);
            let output_buffer;
            match args.file_type {
                FileTypes::Progress => {
                    output_buffer = cryptor.decrypt_progress()?;
                }
                FileTypes::Contraption => {
                    output_buffer = cryptor.decrypt_contraption()?;
                }
            }
            let mut output_file = File::create(args.output_file)?;
            output_file.write_all(&output_buffer)?;
        }
        Commands::Generate(args) => {
            let output_file = args.get_file();
            match Resource::get_example() {
                Some(file) => {
                    let mut output_file = File::create(output_file)?;
                    output_file.write_all(&file)?;
                }
                None => {
                    eprintln!("Cannot create example file.")
                }
            }
        }
    }

    Ok(())
}
