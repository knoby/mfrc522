#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MifareCommand {
    AuthWithKeyA = 0x60,
    AuthWithKeyB = 0x61,
    Read = 0x30,
    Write = 0xA0,
    Decrement = 0xC0,
    Increment = 0xC1,
    Restore = 0xC2,
    Transfer = 0xB0,
}

impl MifareCommand {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

/// Enum to determ which key is used for authentication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// Key A of a MifareClassic
    KeyA([u8; 6]),
    /// Key B of a MifareClassic
    KeyB([u8; 6]),
}

impl Key {
    pub fn default_key_a() -> Self {
        Key::KeyA([0xff; 6])
    }
    pub fn default_key_b() -> Self {
        Key::KeyB([0xff; 6])
    }
}
