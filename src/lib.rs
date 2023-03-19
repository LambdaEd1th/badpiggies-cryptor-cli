pub mod cli;
pub mod crypto;

pub mod constant_items {
    pub const SALT: &[u8] = &[
        0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
    ];
    pub const PROGRESS_PASSWORD: &[u8] = b"56SA%FG42Dv5#4aG67f2";
    pub const CONTRAPTION_PASSWORD: &[u8] = b"3b91A049Ca7HvSjhxT35";
}
