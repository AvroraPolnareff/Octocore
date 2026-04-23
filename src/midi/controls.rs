use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum_macros::EnumIter;
use typenum::{IsLessOrEqual, U4, U8};

use crate::ui::button::Button;

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, FromPrimitive)]
pub enum TrackIndex {
    T1 = 0,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, FromPrimitive)]
pub enum Duration {
    D1_4 = 0,
    D1_4t,
    D1_8,
    D1_8t,
    D1_16,
    D1_16t,
    D1_32,
    D1_32t,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PushButton {
    UpperRow(TrackIndex),
    LowerRow(TrackIndex),
    RepeatTime(Duration),
    TapTempo,
    Metronome,
    Delete,
    Undo,
    Mute,
    Solo,
    Stop,
    Convert,
    DoubleLoop,
    Quantize,
    Duplicate,
    New,
    FixedLength,
    Automate,
    Record,
    Play,
    Setup,
    User,
    AddDevice,
    AddTrack,
    Device,
    Mix,
    Browse,
    Clip,
    Master,
    ControlUp,
    ControlDown,
    ControlLeft,
    ControlRight,
    Repeat,
    Accent,
    Scale,
    Layout,
    Note,
    Session,
    OctaveUp,
    OctaveDown,
    PagePrev,
    PageNext,
    Shift,
    Select,
}

impl PushButton {
    pub fn to_midi(&self) -> [u8; 2] {
        match self {
            // TODO: make return u8 instead
            Self::UpperRow(btn) => [0xB0, btn.clone() as u8 + 102],
            Self::LowerRow(btn) => [0xB0, btn.clone() as u8 + 20],
            Self::RepeatTime(btn) => [0xB0, btn.clone() as u8 + 36],
            Self::TapTempo => [0xB0, 3],
            Self::Metronome => [0xB0, 9],
            Self::Delete => [0xB0, 118],
            Self::Undo => [0xB0, 119],
            Self::Mute => [0xB0, 60],
            Self::Solo => [0xB0, 61],
            Self::Stop => [0xB0, 29],
            Self::Convert => [0xB0, 35],
            Self::DoubleLoop => [0xB0, 117],
            Self::Quantize => [0xB0, 116],
            Self::Duplicate => [0xB0, 88],
            Self::New => [0xB0, 87],
            Self::FixedLength => [0xB0, 90],
            Self::Automate => [0xB0, 89],
            Self::Record => [0xB0, 86],
            Self::Play => [0xB0, 85],
            Self::Setup => [0xB0, 30],
            Self::User => [0xB0, 59],
            Self::AddDevice => [0xB0, 52],
            Self::AddTrack => [0xB0, 53],
            Self::Device => [0xB0, 110],
            Self::Mix => [0xB0, 112],
            Self::Browse => [0xB0, 111],
            Self::Clip => [0xB0, 113],
            Self::Master => [0xB0, 28],
            Self::ControlUp => [0xB0, 46],
            Self::ControlDown => [0xB0, 47],
            Self::ControlLeft => [0xB0, 44],
            Self::ControlRight => [0xB0, 45],
            Self::Repeat => [0xB0, 56],
            Self::Accent => [0xB0, 57],
            Self::Scale => [0xB0, 58],
            Self::Layout => [0xB0, 31],
            Self::Note => [0xB0, 50],
            Self::Session => [0xB0, 51],
            Self::OctaveUp => [0xB0, 55],
            Self::OctaveDown => [0xB0, 54],
            Self::PagePrev => [0xB0, 62],
            Self::PageNext => [0xB0, 63],
            Self::Shift => [0xB0, 49],
            Self::Select => [0xB0, 48],
        }
    }

    pub fn from_midi(control_number: u8) -> Option<Self> {
        match control_number {
            (102..=109) => Some(Self::UpperRow(
                TrackIndex::from_u8(control_number.clone() - 102).unwrap(),
            )),
            (20..=27) => Some(Self::LowerRow(
                TrackIndex::from_u8(control_number.clone() - 20).unwrap(),
            )),
            (36..=43) => Some(Self::RepeatTime(
                Duration::from_u8(control_number.clone() - 36).unwrap(),
            )),
            3 => Some(Self::TapTempo),
            9 => Some(Self::Metronome),
            118 => Some(Self::Delete),
            119 => Some(Self::Undo),
            60 => Some(Self::Mute),
            61 => Some(Self::Solo),
            29 => Some(Self::Stop),
            35 => Some(Self::Convert),
            117 => Some(Self::DoubleLoop),
            116 => Some(Self::Quantize),
            88 => Some(Self::Duplicate),
            87 => Some(Self::New),
            90 => Some(Self::FixedLength),
            89 => Some(Self::Automate),
            86 => Some(Self::Record),
            85 => Some(Self::Play),
            30 => Some(Self::Setup),
            59 => Some(Self::User),
            52 => Some(Self::AddDevice),
            53 => Some(Self::AddTrack),
            110 => Some(Self::Device),
            112 => Some(Self::Mix),
            111 => Some(Self::Browse),
            113 => Some(Self::Clip),
            28 => Some(Self::Master),
            46 => Some(Self::ControlUp),
            47 => Some(Self::ControlDown),
            44 => Some(Self::ControlLeft),
            45 => Some(Self::ControlRight),
            56 => Some(Self::Repeat),
            57 => Some(Self::Accent),
            58 => Some(Self::Scale),
            31 => Some(Self::Layout),
            50 => Some(Self::Note),
            51 => Some(Self::Session),
            55 => Some(Self::OctaveUp),
            54 => Some(Self::OctaveDown),
            62 => Some(Self::PagePrev),
            63 => Some(Self::PageNext),
            49 => Some(Self::Shift),
            48 => Some(Self::Select),

            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PushPad(u8);

impl PushPad {
    pub fn new(pad: u8) -> Self {
        if pad > 63 {
            panic!("Push has only 64 pads")
        }
        Self(pad)
    }
    pub fn to_midi(&self) -> u8 {
        self.0 + 36
    }
    pub fn from_midi(control_number: u8) -> Option<Self> {
        match control_number {
            (36..=99) => Some(PushPad::new(control_number - 36)),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ButtonMessage {
    pub button: PushButton,
    pub pressed: bool,
}

impl ButtonMessage {
    pub fn from_midi(message: &[u8]) -> Option<Self> {
        if let [control_kind, control_number, state] = message {
            match control_kind {
                0xB0 => match PushButton::from_midi(*control_number) {
                    Some(btn) => Some(Self {
                        button: btn,
                        pressed: *state > 0,
                    }),
                    None => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PadMessage {
    pub pad: PushPad,
    pub velocity: u8,
    pub pressed: bool,
}

impl PadMessage {
    pub fn from_midi(message: &[u8]) -> Option<Self> {
        if let [control_kind, control_number, state] = message {
            match control_kind {
                0x90 => match PushPad::from_midi(*control_number) {
                    Some(pad) => Some(Self {
                        pad: pad,
                        pressed: true,
                        velocity: *state,
                    }),
                    None => None,
                },
                0x80 => match PushPad::from_midi(*control_number) {
                    Some(pad) => Some(Self {
                        pad: pad,
                        pressed: false,
                        velocity: *state,
                    }),
                    None => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PushMessage {
    PadPress(PadMessage),
    ButtonPress(ButtonMessage),
}

impl PushMessage {
    pub fn from_midi(message: &[u8]) -> Option<Self> {
        if let Some(pad) = PadMessage::from_midi(message) {
            Some(Self::PadPress(pad))
        } else if let Some(btn) = ButtonMessage::from_midi(message) {
            Some(Self::ButtonPress(btn))
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_midi_converting() {
        assert_eq!(PushButton::UpperRow(TrackIndex::T1).to_midi(), [0xB0, 102]);
        assert_eq!(PushButton::UpperRow(TrackIndex::T8).to_midi(), [0xB0, 109]);
        assert_eq!(PushButton::LowerRow(TrackIndex::T1).to_midi(), [0xB0, 20]);
        assert_eq!(PushButton::LowerRow(TrackIndex::T8).to_midi(), [0xB0, 27]);
    }
    #[test]
    fn from_midi_converting() {
        assert_eq!(
            PushButton::from_midi(102),
            Some(PushButton::UpperRow(TrackIndex::T1)),
        );
        assert_eq!(
            PushButton::from_midi(109),
            Some(PushButton::UpperRow(TrackIndex::T8)),
        );
        assert_eq!(
            PushButton::from_midi(20),
            Some(PushButton::LowerRow(TrackIndex::T1)),
        );
        assert_eq!(
            PushButton::from_midi(26),
            Some(PushButton::LowerRow(TrackIndex::T7)),
        );
        assert_eq!(
            PushButton::from_midi(27),
            Some(PushButton::LowerRow(TrackIndex::T8)),
        );
    }
}
