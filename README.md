# Bad Piggies Cryptor CLI

A fast, cross-platform command-line utility built with Rust for managing (encrypting and decrypting) user data files from the game **Bad Piggies**.

This tool enables users to modify game progress (`Progress.dat`) and facilitate the sharing or editing of vehicle blueprints (`.contraption`) by reliably converting files between the game's encrypted binary format and a human-readable XML format.

## Features

* **Decrypt Game Progress**: Convert the encrypted `Progress.dat` file into editable XML format.
* **Encrypt Game Progress**: Convert an edited XML file back into the encrypted binary `Progress.dat` that the game can read.
* **Contraption Blueprint Support**: Encrypt and decrypt individual vehicle blueprint files (`.contraption`).
* **Generate Template**: Quickly create a template `Progress.dat.xml` file for starting new saves or testing.
* **Cross-Platform**: Compatible with Windows, macOS, and Linux.

## Installation

### Download Binaries
Pre-compiled executables for various platforms are available on the [Releases page](https://github.com/LambdaEd1th/badpiggies-cryptor-cli/releases).

### Build from Source
If you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed, you can build the project yourself:

```bash
git clone [https://github.com/LambdaEd1th/badpiggies-cryptor-cli.git](https://github.com/LambdaEd1th/badpiggies-cryptor-cli.git)
cd badpiggies-cryptor-cli
cargo build --release
````

The compiled executable will be located in the `target/release/` directory.

## Usage

The application uses a standard `COMMAND [OPTIONS] <FILE_TYPE>` structure.

```bash
badpiggies-cryptor-cli <COMMAND> [OPTIONS]
```

### Commands

| Command | Description |
| :--- | :--- |
| `decrypt` | Decrypts a binary file (Binary -\> XML). |
| `encrypt` | Encrypts a source file (XML -\> Binary). |
| `generate` | Generates a sample `Progress.dat.xml` file. |

### Arguments

| Argument | Description |
| :--- | :--- |
| `-i, --input <PATH>` | The source file path for processing. |
| `-o, --output <PATH>` | The destination path for the result (optional). |
| `<FILE_TYPE>` | The file format being processed. Must be either `progress` or `contraption`. |

### File Types

| File Type | Description |
| :--- | :--- |
| `progress` | For the main game save file (`Progress.dat`). |
| `contraption` | For individual vehicle blueprints (`.contraption`). |

-----

## Examples

### 1\. Decrypt Game Save

Convert an encrypted `Progress.dat` into `Progress.dat.xml` for editing coins, power-ups, or unlocked levels.

```bash
badpiggies-cryptor-cli decrypt -i Progress.dat -o Progress.dat.xml progress
```

### 2\. Encrypt Modified Save

Re-encrypt the edited XML file back to `Progress.dat` for use in the game.

```bash
badpiggies-cryptor-cli encrypt -i Progress.dat.xml -o Progress.dat progress
```

### 3\. Decrypt a Contraption Blueprint

Convert a `.contraption` file into an XML format.

```bash
badpiggies-cryptor-cli decrypt -i my_vehicle.contraption -o my_vehicle.xml contraption
```

### 4\. Generate a Sample XML

Create a default template save file named `Progress.dat.xml` in the current directory.

```bash
badpiggies-cryptor-cli generate
```

-----

## Technical Specifications

The tool accurately implements the custom encryption scheme used by Bad Piggies:

  * **Algorithm**: **AES-256-CBC** with PKCS7 padding.
  * **Key Derivation**: **PBKDF2-HMAC-SHA1** with **1000 iterations**.
  * **Data Integrity**: `Progress.dat` files include a **20-byte SHA1 checksum header** placed before the AES ciphertext to ensure file integrity and detect tampering.

-----

## License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for the full text.

-----

***Disclaimer**: This tool is an unofficial utility and is neither affiliated with nor endorsed by Rovio Entertainment. Use it at your own risk. Always back up your save files before modification.*
