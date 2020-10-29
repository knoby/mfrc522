// Commands
pub const REQA: u8 = 0x26;
pub const WUPA: u8 = 0x52;
pub const HLTA: [u8; 2] = [0x55, 0x00];
pub const AUTHKEYA: u8 = 0x60;
pub const AUTHKEYB: u8 = 0x61;

// Cascade levels
pub const SEL_CL1: u8 = 0x93;
pub const SEL_CL2: u8 = 0x95;
pub const SEL_CL3: u8 = 0x97;

// Cascade tag
pub const CT: u8 = 0x88;

// Auth Key
pub const STANDRAD_KEY: [u8; 6] = [0xff; 6];
