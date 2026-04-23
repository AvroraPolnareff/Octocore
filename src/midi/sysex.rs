#[derive(Debug, Clone, PartialEq)]
pub struct SysexMessage(pub Vec<u8>);

impl SysexMessage {
    pub fn to_sysex(command: Vec<u8>) -> SysexMessage {
        let sysex_header = vec![0xF0, 0x00, 0x21, 0x1D, 0x01, 0x01];
        let sysex_end = 0xF7;
        SysexMessage(vec![sysex_header, command, vec![sysex_end]].concat())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PaletteColor {
    pub index: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
}

impl PaletteColor {
    pub fn to_sysex(&self) -> SysexMessage {
        let set_color_entry = 0x03;
        SysexMessage::to_sysex(vec![
            set_color_entry,
            self.index & 0x7F,
            self.red & 0x7F,
            (self.red >> 7) & 0x01,
            self.green & 0x7F,
            (self.green >> 7) & 0x01,
            self.blue & 0x7F,
            (self.blue >> 7) & 0x01,
            self.white & 0x7F,
            (self.white >> 7) & 0x01,
        ])
    }
}
