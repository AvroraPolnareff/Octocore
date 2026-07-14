use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use strum_macros::EnumIter;

use crate::{
    midi::controls::{PushButton, PushPad},
    ui::widget::Pad,
};

/*
 * Speed in less than one divisions
 * Tempo controlled by MIDI sync message
 */
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, ToPrimitive)]
pub enum AnimationSpeed {
    q24 = 0,
    q16,
    q8,
    q4,
    q2,
}

/*
 * Animation types
 */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LedAnimation {
    None,                     // No animation
    OneShot(AnimationSpeed),  // Ramp from zero to max
    Pulsing(AnimationSpeed),  // Triangle-shaped transition
    Blinking(AnimationSpeed), // Square-shaped transition
}
impl LedAnimation {
    /*
     * Returns animation as MIDI Channel
     */
    pub fn to_midi(&self) -> u8 {
        match self {
            LedAnimation::None => 0,
            LedAnimation::OneShot(length) => length.to_u8().unwrap_or(0) + 1,
            LedAnimation::Blinking(length) => length.to_u8().unwrap_or(0) + 6,
            LedAnimation::Pulsing(length) => length.to_u8().unwrap_or(0) + 11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub u8, pub LedAnimation);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColoredControl {
    Button(PushButton),
    Pad(PushPad),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMessage {
    pub color: Color,
    pub control: ColoredControl,
}
impl ColorMessage {
    pub fn to_midi(&self) -> Vec<u8> {
        let [control_kind, control] = match self.control {
            ColoredControl::Pad(pad) => [0x90, pad.to_midi()],
            ColoredControl::Button(btn) => btn.to_midi(),
        };
        let Color(color_index, animation) = self.color;
        let animation = animation.to_midi();
        vec![control_kind + animation, control, color_index]
    }
}
