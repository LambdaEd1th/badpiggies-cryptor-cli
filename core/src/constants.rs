/// Number of iterations for PBKDF2 key derivation.
pub const PBKDF2_ITERATIONS: u32 = 1000;
/// Length of the SHA1 checksum header in bytes.
pub const SHA1_HEADER_LEN: usize = 20;
/// Length of the AES-256 key in bytes.
pub const KEY_LEN: usize = 32;
/// Length of the Initialization Vector (IV) in bytes.
pub const IV_LEN: usize = 16;
/// Total length of bytes needed from PBKDF2 (Key + IV).
pub const DERIVED_LEN: usize = KEY_LEN + IV_LEN;

pub const SALT: &[u8] = &[
    0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
];
// Hardcoded passwords used by the game
pub const PROGRESS_PWD: &[u8] = b"56SA%FG42Dv5#4aG67f2";
pub const CONTRAPTION_PWD: &[u8] = b"3b91A049Ca7HvSjhxT35";

pub const TEMPLATE_XML: &[u8] = include_bytes!("../../tests/template.xml");
