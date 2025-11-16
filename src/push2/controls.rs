use crate::push2::midi::*;
/// Control mapping for Ableton Push 2
///
/// This module provides type-safe mapping between physical Push 2 controls
/// and their MIDI representations according to the official specification.
use std::fmt;

/// All physical controls on Push 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Push2Control {
    // Pads (8x8 grid, 64 total)
    Pad { row: u8, col: u8 },

    // Track encoders (1-8)
    TrackEncoder(u8),

    // Master encoder
    MasterEncoder,

    // Tempo and Swing encoders
    TempoEncoder,
    SwingEncoder,

    // Track buttons (below display, 1-8)
    TrackButton(u8),

    // Track buttons (above display, 1-8)
    TrackButtonAbove(u8),

    // Master button
    MasterButton,

    // Scene buttons (1-8)
    SceneButton(u8),

    // Navigation buttons
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    SelectButton,

    // Mode buttons
    ShiftButton,
    NoteButton,
    SessionButton,
    AddDeviceButton,
    AddTrackButton,

    // Octave buttons
    OctaveDown,
    OctaveUp,

    // Performance buttons
    RepeatButton,
    AccentButton,
    ScaleButton,
    UserButton,
    MuteButton,
    SoloButton,
    PageLeft,
    PageRight,

    // Transport buttons
    PlayButton,
    RecordButton,
    NewButton,
    DuplicateButton,
    AutomateButton,
    FixedLengthButton,

    // Setup buttons
    StopClipButton,
    SetupButton,
    LayoutButton,
    ConvertButton,
    MetronomeButton,
    TapTempoButton,

    // Device/Browse/Mix/Clip buttons
    DeviceButton,
    BrowseButton,
    MixButton,
    ClipButton,

    // Other buttons
    QuantizeButton,
    DoubleLoopButton,
    DeleteButton,
    UndoButton,

    // Touch strip
    TouchStrip,
}

impl Push2Control {
    /// Convert MIDI note number (36-99) to pad coordinates
    /// Returns (row, col) where (0,0) is bottom-left
    pub fn note_to_pad_coords(note: u8) -> Option<(u8, u8)> {
        if note < PAD_NOTE_MIN || note > PAD_NOTE_MAX {
            return None;
        }
        let pad_index = note - PAD_NOTE_MIN;
        let row = pad_index / 8;
        let col = pad_index % 8;
        // Push 2 pads are arranged with (0,0) at bottom-left
        // So we need to flip the row
        Some((7 - row, col))
    }

    /// Convert pad coordinates to MIDI note number
    pub fn pad_coords_to_note(row: u8, col: u8) -> Option<u8> {
        if row > 7 || col > 7 {
            return None;
        }
        // Flip row to match Push 2 layout
        let flipped_row = 7 - row;
        Some(PAD_NOTE_MIN + (flipped_row * 8) + col)
    }

    /// Get MIDI CC number for this control
    pub fn to_cc(&self) -> Option<u8> {
        match self {
            // Track encoders
            Push2Control::TrackEncoder(n) if *n >= 1 && *n <= 8 => {
                Some((CC_TRACK_ENCODER_1 - 1) + n)
            }

            // Master encoder
            Push2Control::MasterEncoder => Some(CC_MASTER_ENCODER),

            // Tempo/Swing encoders
            Push2Control::TempoEncoder => Some(CC_TEMPO_ENCODER),
            Push2Control::SwingEncoder => Some(CC_SWING_ENCODER),

            // Track buttons below display
            Push2Control::TrackButton(n) if *n >= 1 && *n <= 8 => Some((CC_TRACK_BUTTON_1 - 1) + n),

            // Track buttons above display
            Push2Control::TrackButtonAbove(n) if *n >= 1 && *n <= 8 => {
                Some((CC_TRACK_BUTTON_ABOVE_1 - 1) + n)
            }

            // Master button
            Push2Control::MasterButton => Some(CC_MASTER_BUTTON),

            // Scene buttons (CC 36-43, no constants defined in midi.rs for these)
            // These are mapped to LSB of various controllers in the spec
            Push2Control::SceneButton(n) if *n >= 1 && *n <= 8 => Some(35 + n),

            // Navigation
            Push2Control::ArrowLeft => Some(CC_ARROW_LEFT),
            Push2Control::ArrowRight => Some(CC_ARROW_RIGHT),
            Push2Control::ArrowUp => Some(CC_ARROW_UP),
            Push2Control::ArrowDown => Some(CC_ARROW_DOWN),
            Push2Control::SelectButton => Some(CC_SELECT_BUTTON),

            // Mode buttons
            Push2Control::ShiftButton => Some(CC_SHIFT_BUTTON),
            Push2Control::NoteButton => Some(CC_NOTE_BUTTON),
            Push2Control::SessionButton => Some(CC_SESSION_BUTTON),
            Push2Control::AddDeviceButton => Some(CC_ADD_DEVICE_BUTTON),
            Push2Control::AddTrackButton => Some(CC_ADD_TRACK_BUTTON),

            // Octave
            Push2Control::OctaveDown => Some(CC_OCTAVE_DOWN),
            Push2Control::OctaveUp => Some(CC_OCTAVE_UP),

            // Performance
            Push2Control::RepeatButton => Some(CC_REPEAT_BUTTON),
            Push2Control::AccentButton => Some(CC_ACCENT_BUTTON),
            Push2Control::ScaleButton => Some(CC_SCALE_BUTTON),
            Push2Control::UserButton => Some(CC_USER_BUTTON),
            Push2Control::MuteButton => Some(CC_MUTE_BUTTON),
            Push2Control::SoloButton => Some(CC_SOLO_BUTTON),
            Push2Control::PageLeft => Some(CC_PAGE_LEFT),
            Push2Control::PageRight => Some(CC_PAGE_RIGHT),

            // Transport
            Push2Control::PlayButton => Some(CC_PLAY_BUTTON),
            Push2Control::RecordButton => Some(CC_RECORD_BUTTON),
            Push2Control::NewButton => Some(CC_NEW_BUTTON),
            Push2Control::DuplicateButton => Some(CC_DUPLICATE_BUTTON),
            Push2Control::AutomateButton => Some(CC_AUTOMATE_BUTTON),
            Push2Control::FixedLengthButton => Some(CC_FIXED_LENGTH_BUTTON),

            // Setup
            Push2Control::StopClipButton => Some(CC_STOP_CLIP_BUTTON),
            Push2Control::SetupButton => Some(CC_SETUP_BUTTON),
            Push2Control::LayoutButton => Some(CC_LAYOUT_BUTTON),
            Push2Control::ConvertButton => Some(CC_CONVERT_BUTTON),
            Push2Control::MetronomeButton => Some(CC_METRONOME),
            Push2Control::TapTempoButton => Some(CC_TAP_TEMPO),

            // Device/Browse/Mix/Clip
            Push2Control::DeviceButton => Some(CC_DEVICE_BUTTON),
            Push2Control::BrowseButton => Some(CC_BROWSE_BUTTON),
            Push2Control::MixButton => Some(CC_MIX_BUTTON),
            Push2Control::ClipButton => Some(CC_CLIP_BUTTON),

            // Other
            Push2Control::QuantizeButton => Some(CC_QUANTIZE_BUTTON),
            Push2Control::DoubleLoopButton => Some(CC_DOUBLE_LOOP_BUTTON),
            Push2Control::DeleteButton => Some(CC_DELETE_BUTTON),
            Push2Control::UndoButton => Some(CC_UNDO_BUTTON),

            // Touch strip
            Push2Control::TouchStrip => Some(CC_TOUCH_STRIP),

            _ => None,
        }
    }

    /// Create control from MIDI CC number
    pub fn from_cc(cc: u8) -> Option<Self> {
        match cc {
            // Track encoders
            CC_TRACK_ENCODER_1..=CC_TRACK_ENCODER_8 => {
                Some(Push2Control::TrackEncoder(cc - (CC_TRACK_ENCODER_1 - 1)))
            }

            // Master encoder
            CC_MASTER_ENCODER => Some(Push2Control::MasterEncoder),

            // Tempo/Swing
            CC_TEMPO_ENCODER => Some(Push2Control::TempoEncoder),
            CC_SWING_ENCODER => Some(Push2Control::SwingEncoder),

            // Track buttons below display
            CC_TRACK_BUTTON_1..=CC_TRACK_BUTTON_8 => {
                Some(Push2Control::TrackButton(cc - (CC_TRACK_BUTTON_1 - 1)))
            }

            // Track buttons above display
            CC_TRACK_BUTTON_ABOVE_1..=CC_TRACK_BUTTON_ABOVE_8 => Some(
                Push2Control::TrackButtonAbove(cc - (CC_TRACK_BUTTON_ABOVE_1 - 1)),
            ),

            // Master
            CC_MASTER_BUTTON => Some(Push2Control::MasterButton),

            // Scene buttons (CC 36-43, no constants defined in midi.rs for these)
            // These are mapped to LSB of various controllers in the spec
            cc if cc >= 36 && cc <= 43 => Some(Push2Control::SceneButton(cc - 35)),

            // Navigation
            CC_ARROW_LEFT => Some(Push2Control::ArrowLeft),
            CC_ARROW_RIGHT => Some(Push2Control::ArrowRight),
            CC_ARROW_UP => Some(Push2Control::ArrowUp),
            CC_ARROW_DOWN => Some(Push2Control::ArrowDown),
            CC_SELECT_BUTTON => Some(Push2Control::SelectButton),

            // Mode
            CC_SHIFT_BUTTON => Some(Push2Control::ShiftButton),
            CC_NOTE_BUTTON => Some(Push2Control::NoteButton),
            CC_SESSION_BUTTON => Some(Push2Control::SessionButton),
            CC_ADD_DEVICE_BUTTON => Some(Push2Control::AddDeviceButton),
            CC_ADD_TRACK_BUTTON => Some(Push2Control::AddTrackButton),

            // Octave
            CC_OCTAVE_DOWN => Some(Push2Control::OctaveDown),
            CC_OCTAVE_UP => Some(Push2Control::OctaveUp),

            // Performance
            CC_REPEAT_BUTTON => Some(Push2Control::RepeatButton),
            CC_ACCENT_BUTTON => Some(Push2Control::AccentButton),
            CC_SCALE_BUTTON => Some(Push2Control::ScaleButton),
            CC_USER_BUTTON => Some(Push2Control::UserButton),
            CC_MUTE_BUTTON => Some(Push2Control::MuteButton),
            CC_SOLO_BUTTON => Some(Push2Control::SoloButton),
            CC_PAGE_LEFT => Some(Push2Control::PageLeft),
            CC_PAGE_RIGHT => Some(Push2Control::PageRight),

            // Transport
            CC_PLAY_BUTTON => Some(Push2Control::PlayButton),
            CC_RECORD_BUTTON => Some(Push2Control::RecordButton),
            CC_NEW_BUTTON => Some(Push2Control::NewButton),
            CC_DUPLICATE_BUTTON => Some(Push2Control::DuplicateButton),
            CC_AUTOMATE_BUTTON => Some(Push2Control::AutomateButton),
            CC_FIXED_LENGTH_BUTTON => Some(Push2Control::FixedLengthButton),

            // Setup
            CC_STOP_CLIP_BUTTON => Some(Push2Control::StopClipButton),
            CC_SETUP_BUTTON => Some(Push2Control::SetupButton),
            CC_LAYOUT_BUTTON => Some(Push2Control::LayoutButton),
            CC_CONVERT_BUTTON => Some(Push2Control::ConvertButton),
            CC_METRONOME => Some(Push2Control::MetronomeButton),
            CC_TAP_TEMPO => Some(Push2Control::TapTempoButton),

            // Device/Browse/Mix/Clip
            CC_DEVICE_BUTTON => Some(Push2Control::DeviceButton),
            CC_BROWSE_BUTTON => Some(Push2Control::BrowseButton),
            CC_MIX_BUTTON => Some(Push2Control::MixButton),
            CC_CLIP_BUTTON => Some(Push2Control::ClipButton),

            // Other
            CC_QUANTIZE_BUTTON => Some(Push2Control::QuantizeButton),
            CC_DOUBLE_LOOP_BUTTON => Some(Push2Control::DoubleLoopButton),
            CC_DELETE_BUTTON => Some(Push2Control::DeleteButton),
            CC_UNDO_BUTTON => Some(Push2Control::UndoButton),

            // Touch strip
            CC_TOUCH_STRIP => Some(Push2Control::TouchStrip),

            _ => None,
        }
    }

    /// Get encoder ID (1-8 for tracks, 9 for master, 10 for tempo, 11 for swing)
    pub fn encoder_id(&self) -> Option<u8> {
        match self {
            Push2Control::TrackEncoder(n) if *n >= 1 && *n <= 8 => Some(*n),
            Push2Control::MasterEncoder => Some(9),
            Push2Control::TempoEncoder => Some(10),
            Push2Control::SwingEncoder => Some(11),
            _ => None,
        }
    }
}

impl fmt::Display for Push2Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Push2Control::Pad { row, col } => write!(f, "Pad({}, {})", row, col),
            Push2Control::TrackEncoder(n) => write!(f, "TrackEncoder({})", n),
            Push2Control::MasterEncoder => write!(f, "MasterEncoder"),
            Push2Control::TempoEncoder => write!(f, "TempoEncoder"),
            Push2Control::SwingEncoder => write!(f, "SwingEncoder"),
            Push2Control::TrackButton(n) => write!(f, "TrackButton({})", n),
            Push2Control::TrackButtonAbove(n) => write!(f, "TrackButtonAbove({})", n),
            Push2Control::MasterButton => write!(f, "MasterButton"),
            Push2Control::SceneButton(n) => write!(f, "SceneButton({})", n),
            Push2Control::ArrowLeft => write!(f, "ArrowLeft"),
            Push2Control::ArrowRight => write!(f, "ArrowRight"),
            Push2Control::ArrowUp => write!(f, "ArrowUp"),
            Push2Control::ArrowDown => write!(f, "ArrowDown"),
            Push2Control::SelectButton => write!(f, "SelectButton"),
            Push2Control::ShiftButton => write!(f, "ShiftButton"),
            Push2Control::NoteButton => write!(f, "NoteButton"),
            Push2Control::SessionButton => write!(f, "SessionButton"),
            Push2Control::AddDeviceButton => write!(f, "AddDeviceButton"),
            Push2Control::AddTrackButton => write!(f, "AddTrackButton"),
            Push2Control::OctaveDown => write!(f, "OctaveDown"),
            Push2Control::OctaveUp => write!(f, "OctaveUp"),
            Push2Control::RepeatButton => write!(f, "RepeatButton"),
            Push2Control::AccentButton => write!(f, "AccentButton"),
            Push2Control::ScaleButton => write!(f, "ScaleButton"),
            Push2Control::UserButton => write!(f, "UserButton"),
            Push2Control::MuteButton => write!(f, "MuteButton"),
            Push2Control::SoloButton => write!(f, "SoloButton"),
            Push2Control::PageLeft => write!(f, "PageLeft"),
            Push2Control::PageRight => write!(f, "PageRight"),
            Push2Control::PlayButton => write!(f, "PlayButton"),
            Push2Control::RecordButton => write!(f, "RecordButton"),
            Push2Control::NewButton => write!(f, "NewButton"),
            Push2Control::DuplicateButton => write!(f, "DuplicateButton"),
            Push2Control::AutomateButton => write!(f, "AutomateButton"),
            Push2Control::FixedLengthButton => write!(f, "FixedLengthButton"),
            Push2Control::StopClipButton => write!(f, "StopClipButton"),
            Push2Control::SetupButton => write!(f, "SetupButton"),
            Push2Control::LayoutButton => write!(f, "LayoutButton"),
            Push2Control::ConvertButton => write!(f, "ConvertButton"),
            Push2Control::MetronomeButton => write!(f, "MetronomeButton"),
            Push2Control::TapTempoButton => write!(f, "TapTempoButton"),
            Push2Control::DeviceButton => write!(f, "DeviceButton"),
            Push2Control::BrowseButton => write!(f, "BrowseButton"),
            Push2Control::MixButton => write!(f, "MixButton"),
            Push2Control::ClipButton => write!(f, "ClipButton"),
            Push2Control::QuantizeButton => write!(f, "QuantizeButton"),
            Push2Control::DoubleLoopButton => write!(f, "DoubleLoopButton"),
            Push2Control::DeleteButton => write!(f, "DeleteButton"),
            Push2Control::UndoButton => write!(f, "UndoButton"),
            Push2Control::TouchStrip => write!(f, "TouchStrip"),
        }
    }
}
