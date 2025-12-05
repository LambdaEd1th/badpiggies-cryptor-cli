# Bad Piggies Cryptor CLI

A command-line utility for encrypting and decrypting user data files from the game **Bad Piggies**.

This tool allows you to modify game progress (`Progress.dat`) and share or edit vehicle blueprints (`.contraption`) by converting them between their encrypted binary format and readable XML.

## Features

* **Decrypt Progress**: Convert `Progress.dat` files into readable XML format for editing.
* **Encrypt Progress**: Convert edited XML files back into the game's binary `Progress.dat` format.
* **Contraption Support**: Encrypt and decrypt vehicle blueprint files (`.contraption`).
* **Generator**: Generate a template `Progress.dat.xml` file for testing or fresh saves.
* **Cross-Platform**: Runs on Windows, macOS, and Linux.

## Installation

### Download Binaries
You can download the pre-compiled binaries for your platform from the [Releases](https://github.com/LambdaEd1th/badpiggies-cryptor-cli/releases) page.

### Build from Source
If you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed, you can build the project from source:

```bash
git clone [https://github.com/LambdaEd1th/badpiggies-cryptor-cli.git](https://github.com/LambdaEd1th/badpiggies-cryptor-cli.git)
cd badpiggies-cryptor-cli
cargo build --release
````

The executable will be located in `target/release/`.

## Usage

```bash
badpiggies-cryptor-cli <COMMAND> [OPTIONS]
```

### Commands

  * `encrypt`: Encrypt a file (XML -\> Binary).
  * `decrypt`: Decrypt a file (Binary -\> XML).
  * `generate`: Generate an example `Progress.dat.xml` file.

### Arguments

  * `-i, --input-file <PATH>`: The source file to process.
  * `-o, --output-file <PATH>`: The destination path for the result.
  * `<FILE_TYPE>`: The type of file being processed.
      * `progress`: For game save files (e.g., `Progress.dat`).
      * `contraption`: For vehicle blueprints (e.g., `car.contraption`).

## Examples

### 1\. Decrypting a Save File

Convert an encrypted `Progress.dat` file into an XML file to edit your coins, unlocked levels, or parts.

```bash
badpiggies-cryptor-cli decrypt -i Progress.dat -o Progress.dat.xml progress
```

### 2\. Encrypting a Save File

After editing your XML file, convert it back to the format the game can read.

```bash
badpiggies-cryptor-cli encrypt -i Progress.dat.xml -o Progress.dat progress
```

### 3\. Decrypting a Contraption

Convert a vehicle file to XML.

```bash
badpiggies-cryptor-cli decrypt -i my_vehicle.contraption -o my_vehicle.xml contraption
```

### 4\. Generating a Template

Create a default sample XML file in the current directory.

```bash
badpiggies-cryptor-cli generate
```

## Technical Details

This tool handles the specific encryption scheme used by Bad Piggies:

  * **Algorithm**: AES-256-CBC.
  * **Key Derivation**: PBKDF2-HMAC-SHA1 (1000 iterations).
  * **Integrity**: Progress files include a SHA1 checksum header.

## License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

-----

*Disclaimer: This tool is an unofficial utility and is not affiliated with or endorsed by Rovio Entertainment. Use it at your own risk. Always backup your save files before modification.*
