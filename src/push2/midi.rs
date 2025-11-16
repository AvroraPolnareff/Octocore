/// MIDI protocol constants and utilities for Push 2
///
/// This module provides MIDI message handling according to the
/// Ableton Push 2 MIDI specification.

/// MIDI note range for encoder touches
/// Encoders send Note On/Off when touched: 0-7 for track encoders, 8 for master, 9 for tempo, 10 for swing
pub const ENCODER_TOUCH_NOTE_MIN: u8 = 0;
pub const ENCODER_TOUCH_NOTE_MAX: u8 = 10;
pub const ENCODER_TOUCH_COUNT: u8 = 11;

/// MIDI note range for pads (8x8 grid)
pub const PAD_NOTE_MIN: u8 = 36;
pub const PAD_NOTE_MAX: u8 = 99;
pub const PAD_COUNT: u8 = 64;

/// MIDI CC numbers for encoders
pub const CC_TRACK_ENCODER_1: u8 = 71;
pub const CC_TRACK_ENCODER_2: u8 = 72;
pub const CC_TRACK_ENCODER_3: u8 = 73;
pub const CC_TRACK_ENCODER_4: u8 = 74;
pub const CC_TRACK_ENCODER_5: u8 = 75;
pub const CC_TRACK_ENCODER_6: u8 = 76;
pub const CC_TRACK_ENCODER_7: u8 = 77;
pub const CC_TRACK_ENCODER_8: u8 = 78;
pub const CC_MASTER_ENCODER: u8 = 79;
pub const CC_TEMPO_ENCODER: u8 = 14;
pub const CC_SWING_ENCODER: u8 = 15;

/// MIDI CC numbers for buttons
pub const CC_TRACK_BUTTON_1: u8 = 20;
pub const CC_TRACK_BUTTON_2: u8 = 21;
pub const CC_TRACK_BUTTON_3: u8 = 22;
pub const CC_TRACK_BUTTON_4: u8 = 23;
pub const CC_TRACK_BUTTON_5: u8 = 24;
pub const CC_TRACK_BUTTON_6: u8 = 25;
pub const CC_TRACK_BUTTON_7: u8 = 26;
pub const CC_TRACK_BUTTON_8: u8 = 27;
pub const CC_MASTER_BUTTON: u8 = 28;
pub const CC_STOP_CLIP_BUTTON: u8 = 29;
pub const CC_SETUP_BUTTON: u8 = 30;
pub const CC_LAYOUT_BUTTON: u8 = 31;
pub const CC_CONVERT_BUTTON: u8 = 35;

/// MIDI CC numbers for track buttons above display
pub const CC_TRACK_BUTTON_ABOVE_1: u8 = 102;
pub const CC_TRACK_BUTTON_ABOVE_2: u8 = 103;
pub const CC_TRACK_BUTTON_ABOVE_3: u8 = 104;
pub const CC_TRACK_BUTTON_ABOVE_4: u8 = 105;
pub const CC_TRACK_BUTTON_ABOVE_5: u8 = 106;
pub const CC_TRACK_BUTTON_ABOVE_6: u8 = 107;
pub const CC_TRACK_BUTTON_ABOVE_7: u8 = 108;
pub const CC_TRACK_BUTTON_ABOVE_8: u8 = 109;

/// MIDI CC numbers for other controls
pub const CC_TOUCH_STRIP: u8 = 1;
pub const CC_TAP_TEMPO: u8 = 3;
pub const CC_METRONOME: u8 = 9;
pub const CC_ARROW_LEFT: u8 = 44;
pub const CC_ARROW_RIGHT: u8 = 45;
pub const CC_ARROW_UP: u8 = 46;
pub const CC_ARROW_DOWN: u8 = 47;
pub const CC_SELECT_BUTTON: u8 = 48;
pub const CC_SHIFT_BUTTON: u8 = 49;
pub const CC_NOTE_BUTTON: u8 = 50;
pub const CC_SESSION_BUTTON: u8 = 51;
pub const CC_ADD_DEVICE_BUTTON: u8 = 52;
pub const CC_ADD_TRACK_BUTTON: u8 = 53;
pub const CC_OCTAVE_DOWN: u8 = 54;
pub const CC_OCTAVE_UP: u8 = 55;
pub const CC_REPEAT_BUTTON: u8 = 56;
pub const CC_ACCENT_BUTTON: u8 = 57;
pub const CC_SCALE_BUTTON: u8 = 58;
pub const CC_USER_BUTTON: u8 = 59;
pub const CC_MUTE_BUTTON: u8 = 60;
pub const CC_SOLO_BUTTON: u8 = 61;
pub const CC_PAGE_LEFT: u8 = 62;
pub const CC_PAGE_RIGHT: u8 = 63;
pub const CC_PLAY_BUTTON: u8 = 85;
pub const CC_RECORD_BUTTON: u8 = 86;
pub const CC_NEW_BUTTON: u8 = 87;
pub const CC_DUPLICATE_BUTTON: u8 = 88;
pub const CC_AUTOMATE_BUTTON: u8 = 89;
pub const CC_FIXED_LENGTH_BUTTON: u8 = 90;
pub const CC_DEVICE_BUTTON: u8 = 110;
pub const CC_BROWSE_BUTTON: u8 = 111;
pub const CC_MIX_BUTTON: u8 = 112;
pub const CC_CLIP_BUTTON: u8 = 113;
pub const CC_QUANTIZE_BUTTON: u8 = 116;
pub const CC_DOUBLE_LOOP_BUTTON: u8 = 117;
pub const CC_DELETE_BUTTON: u8 = 118;
pub const CC_UNDO_BUTTON: u8 = 119;

/// MIDI channel used by Push 2 (Channel 1, 0-indexed)
pub const PUSH2_MIDI_CHANNEL: u8 = 0;

/// Encoder relative value range
/// Encoders send relative values from -63 to +63
pub const ENCODER_DELTA_MIN: i8 = -63;
pub const ENCODER_DELTA_MAX: i8 = 63;

/// Convert encoder MIDI value to relative delta
/// MIDI values 0-63 are positive, 64-127 are negative (wrapped)
pub fn encoder_value_to_delta(value: u8) -> i8 {
    if value > 64 {
        // Negative: 65-127 maps to -63 to -1
        -((128 - value) as i8)
    } else {
        // Positive: 0-64 maps to 0 to 64 (but spec says max is 63)
        (value.min(63)) as i8
    }
}

/// Convert relative delta to encoder MIDI value
pub fn encoder_delta_to_value(delta: i8) -> u8 {
    if delta < 0 {
        // Negative: -63 to -1 maps to 65-127
        (128i16 + delta as i16) as u8
    } else {
        // Positive: 0-63 maps to 0-63
        delta.min(63) as u8
    }
}

/// Check if a note number is an encoder touch
pub fn is_encoder_touch_note(note: u8) -> bool {
    note >= ENCODER_TOUCH_NOTE_MIN && note <= ENCODER_TOUCH_NOTE_MAX
}

/// Convert encoder touch note to encoder ID
/// Notes 0-7 = track encoders 1-8, 8 = master, 9 = tempo, 10 = swing
pub fn encoder_touch_note_to_id(note: u8) -> Option<u8> {
    if note <= 7 {
        Some(note + 1) // 0-7 -> 1-8
    } else if note == 8 {
        Some(9) // master
    } else if note == 9 {
        Some(10) // tempo
    } else if note == 10 {
        Some(11) // swing
    } else {
        None
    }
}

/// Check if a note number is in the pad range
pub fn is_pad_note(note: u8) -> bool {
    note >= PAD_NOTE_MIN && note <= PAD_NOTE_MAX
}

/// Check if a CC number is an encoder
pub fn is_encoder_cc(cc: u8) -> bool {
    matches!(
        cc,
        CC_TRACK_ENCODER_1
            | CC_TRACK_ENCODER_2
            | CC_TRACK_ENCODER_3
            | CC_TRACK_ENCODER_4
            | CC_TRACK_ENCODER_5
            | CC_TRACK_ENCODER_6
            | CC_TRACK_ENCODER_7
            | CC_TRACK_ENCODER_8
            | CC_MASTER_ENCODER
            | CC_TEMPO_ENCODER
            | CC_SWING_ENCODER
    )
}

/// Get track encoder number from CC (1-8, or None)
pub fn cc_to_track_encoder(cc: u8) -> Option<u8> {
    match cc {
        CC_TRACK_ENCODER_1 => Some(1),
        CC_TRACK_ENCODER_2 => Some(2),
        CC_TRACK_ENCODER_3 => Some(3),
        CC_TRACK_ENCODER_4 => Some(4),
        CC_TRACK_ENCODER_5 => Some(5),
        CC_TRACK_ENCODER_6 => Some(6),
        CC_TRACK_ENCODER_7 => Some(7),
        CC_TRACK_ENCODER_8 => Some(8),
        _ => None,
    }
}
