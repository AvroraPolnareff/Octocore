/// RGB LED Color Control for Push 2
///
/// This module provides color palette management and LED control
/// according to the Ableton Push 2 MIDI specification.
///
/// Reference: https://github.com/Ableton/push-interface/blob/main/doc/AbletonPush2MIDIDisplayInterface.asc#RGB%20LED%20Color%20Processing

use crate::push2::controls::Push2Control;
use crate::push2::midi::{PAD_NOTE_MIN, CC_TOUCH_STRIP};
use midir::MidiOutputConnection;

/// Maximum number of colors in the palette
pub const MAX_PALETTE_COLORS: usize = 127;

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create from HSV color space
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }

    /// Common color constants
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255 };
    pub const ORANGE: Self = Self { r: 255, g: 128, b: 0 };
    pub const PURPLE: Self = Self { r: 128, g: 0, b: 255 };
}

/// LED animation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedAnimation {
    /// Static color (no animation)
    Static,
    /// Pulsing animation
    Pulse,
    /// Blinking animation
    Blink,
}

impl LedAnimation {
    /// Convert to velocity value for Note On message
    /// According to Push 2 spec:
    /// - Velocity 0 = off
    /// - Velocity 1-127 = color index in palette (for static colors)
    /// - For animations, velocity values 1-7 control animation speed
    fn to_velocity(self, color_index: u8) -> u8 {
        match self {
            LedAnimation::Static => {
                // Static color: use color index directly (1-127)
                color_index.max(1).min(127)
            }
            LedAnimation::Pulse => {
                // Pulse animation: velocity 1-7 controls speed
                // We use the color index to determine which color, but velocity controls animation
                // Note: This may need adjustment based on actual Push 2 behavior
                let speed = (color_index % 7 + 1).min(7);
                speed
            }
            LedAnimation::Blink => {
                // Blink animation: velocity 9-15 (1-7 + 8) controls speed
                let speed = (color_index % 7 + 1).min(7);
                speed + 8
            }
        }
    }
}

/// Color palette for Push 2
/// Can contain up to 127 colors
pub struct ColorPalette {
    colors: Vec<RgbColor>,
}

impl ColorPalette {
    /// Get all colors as a slice (for internal use)
    pub(crate) fn colors(&self) -> &[RgbColor] {
        &self.colors
    }
}

impl ColorPalette {
    /// Create a new empty palette
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    /// Create a palette with predefined colors
    pub fn with_colors(colors: Vec<RgbColor>) -> Self {
        Self {
            colors: colors.into_iter().take(MAX_PALETTE_COLORS).collect(),
        }
    }

    /// Add a color to the palette
    /// Returns the index of the added color, or None if palette is full
    pub fn add_color(&mut self, color: RgbColor) -> Option<u8> {
        if self.colors.len() >= MAX_PALETTE_COLORS {
            return None;
        }
        let index = self.colors.len() as u8;
        self.colors.push(color);
        Some(index)
    }

    /// Get color at index
    pub fn get_color(&self, index: u8) -> Option<RgbColor> {
        self.colors.get(index as usize).copied()
    }

    /// Get the number of colors in the palette
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Check if palette is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Clear the palette
    pub fn clear(&mut self) {
        self.colors.clear();
    }

    /// Generate a rainbow palette
    pub fn rainbow(count: usize) -> Self {
        let mut colors = Vec::new();
        for i in 0..count.min(MAX_PALETTE_COLORS) {
            let hue = (i as f32 * 360.0) / count as f32;
            colors.push(RgbColor::from_hsv(hue, 1.0, 1.0));
        }
        Self { colors }
    }

    /// Generate a gradient palette between two colors
    pub fn gradient(from: RgbColor, to: RgbColor, count: usize) -> Self {
        let mut colors = Vec::new();
        for i in 0..count.min(MAX_PALETTE_COLORS) {
            let t = i as f32 / (count - 1).max(1) as f32;
            let r = (from.r as f32 * (1.0 - t) + to.r as f32 * t) as u8;
            let g = (from.g as f32 * (1.0 - t) + to.g as f32 * t) as u8;
            let b = (from.b as f32 * (1.0 - t) + to.b as f32 * t) as u8;
            colors.push(RgbColor::new(r, g, b));
        }
        Self { colors }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new()
    }
}

/// LED Controller for Push 2
pub struct LedController {
    palette: ColorPalette,
}

impl LedController {
    /// Create a new LED controller
    pub fn new() -> Self {
        Self {
            palette: ColorPalette::new(),
        }
    }

    /// Set the color palette
    pub fn set_palette(&mut self, palette: ColorPalette) {
        self.palette = palette;
    }

    /// Get a mutable reference to the palette
    pub fn palette_mut(&mut self) -> &mut ColorPalette {
        &mut self.palette
    }

    /// Get a reference to the palette
    pub fn palette(&self) -> &ColorPalette {
        &self.palette
    }
    
    /// Add a color to the palette and return its index
    /// This is a convenience method that adds the color and returns the index
    pub fn add_color(&mut self, color: RgbColor) -> Option<u8> {
        self.palette.add_color(color)
    }
    
    /// Get or add a color to the palette
    /// If the color already exists, returns its index
    /// Otherwise, adds it and returns the new index
    pub fn get_or_add_color(&mut self, color: RgbColor) -> Option<u8> {
        // Check if color already exists
        for i in 0..self.palette.len() {
            if let Some(existing_color) = self.palette.get_color(i as u8) {
                if existing_color == color {
                    return Some(i as u8);
                }
            }
        }
        // Color doesn't exist, add it
        self.palette.add_color(color)
    }

    /// Send the color palette to Push 2 via SysEx
    /// According to the spec, palette is sent as SysEx with format:
    /// F0 00 21 1D 01 01 03 40 <colors> F7
    /// where colors are RGB triplets (3 bytes per color)
    pub fn send_palette(&self, conn: &mut MidiOutputConnection) -> Result<(), String> {

        let palette_size = self.palette.len();
        if palette_size == 0 {
            return Err("Palette is empty".to_string());
        }

        // SysEx message format for color palette:
        // F0 00 21 1D 01 01 03 40 <num_colors> <colors> F7
        // Ableton vendor ID: 00 21 1D
        // Device ID: 01
        // Command: 01 03 40
        let mut sysex = Vec::new();
        sysex.push(0xF0); // SysEx start
        sysex.push(0x00);
        sysex.push(0x21);
        sysex.push(0x1D); // Ableton vendor ID
        sysex.push(0x01); // Device ID
        sysex.push(0x01); // Command header
        sysex.push(0x03);
        sysex.push(0x40); // Color palette command
        sysex.push(palette_size as u8); // Number of colors

        // Add RGB triplets for each color
        for color in self.palette.colors() {
            sysex.push(color.r);
            sysex.push(color.g);
            sysex.push(color.b);
        }

        sysex.push(0xF7); // SysEx end

        // Send SysEx message
        conn.send(&sysex)
            .map_err(|e| format!("Failed to send palette SysEx: {}", e))?;

        Ok(())
    }

    /// Set pad color and animation
    /// row: 0-7, col: 0-7
    /// color_index: index in palette (0-126)
    pub fn set_pad_color(
        &self,
        conn: &mut MidiOutputConnection,
        row: u8,
        col: u8,
        color_index: u8,
        animation: LedAnimation,
    ) -> Result<(), String> {
        if row > 7 || col > 7 {
            return Err("Pad coordinates out of range".to_string());
        }

        if color_index as usize >= self.palette.len() {
            return Err("Color index out of palette range".to_string());
        }

        // Convert pad coordinates to MIDI note
        let flipped_row = 7 - row;
        let note = PAD_NOTE_MIN + (flipped_row * 8) + col;

        self.send_note_on(conn, note, animation.to_velocity(color_index))
    }

    /// Clear a pad (turn it off)
    pub fn clear_pad(&self, conn: &mut MidiOutputConnection, row: u8, col: u8) -> Result<(), String> {
        if row > 7 || col > 7 {
            return Err("Pad coordinates out of range".to_string());
        }

        let flipped_row = 7 - row;
        let note = PAD_NOTE_MIN + (flipped_row * 8) + col;

        self.send_note_on(conn, note, 0) // Velocity 0 = off
    }

    /// Set button LED state
    /// For buttons, we use CC messages with values:
    /// 0 = off, 1 = on, 2 = blink
    pub fn set_button_led(
        &self,
        conn: &mut MidiOutputConnection,
        control: Push2Control,
        color_index: u8,
        animation: LedAnimation,
    ) -> Result<(), String> {
        let cc = control.to_cc().ok_or("Control does not have CC mapping")?;

        if color_index as usize >= self.palette.len() {
            return Err("Color index out of palette range".to_string());
        }

        // For buttons, we send both:
        // 1. CC message for button state (0=off, 1=on, 2=blink)
        // 2. Note On with color index for RGB color (if supported)

        let cc_value = match animation {
            LedAnimation::Static => 1, // On
            LedAnimation::Blink => 2,  // Blink
            LedAnimation::Pulse => 1,  // On (pulse handled by color)
        };

        // Send CC for button state
        conn.send(&[0xB0, cc, cc_value])
            .map_err(|e| format!("Failed to send button CC: {}", e))?;

        // For RGB buttons, also send Note On with color index
        // Note: Not all buttons support RGB, but we send it anyway
        // The device will ignore it if not supported
        if let Some(note) = self.button_to_note(control) {
            self.send_note_on(conn, note, color_index.max(1).min(127))?;
        }

        Ok(())
    }

    /// Turn off a button
    pub fn clear_button(&self, conn: &mut MidiOutputConnection, control: Push2Control) -> Result<(), String> {
        let cc = control.to_cc().ok_or("Control does not have CC mapping")?;

        // Send CC 0 = off
        conn.send(&[0xB0, cc, 0])
            .map_err(|e| format!("Failed to send button CC: {}", e))?;

        Ok(())
    }

    /// Set touch strip color
    /// Touch strip uses CC 1 with color index as value
    pub fn set_touch_strip_color(&self, conn: &mut MidiOutputConnection, color_index: u8) -> Result<(), String> {
        if color_index as usize >= self.palette.len() {
            return Err("Color index out of palette range".to_string());
        }

        // Touch strip uses CC 1 with color index
        conn.send(&[0xB0, CC_TOUCH_STRIP, color_index.max(1).min(127)])
            .map_err(|e| format!("Failed to send touch strip CC: {}", e))?;

        Ok(())
    }

    /// Clear touch strip
    pub fn clear_touch_strip(&self, conn: &mut MidiOutputConnection) -> Result<(), String> {

        conn.send(&[0xB0, CC_TOUCH_STRIP, 0])
            .map_err(|e| format!("Failed to send touch strip CC: {}", e))?;

        Ok(())
    }

    /// Clear all pads
    pub fn clear_all_pads(&self, conn: &mut MidiOutputConnection) -> Result<(), String> {
        for row in 0..8 {
            for col in 0..8 {
                self.clear_pad(conn, row, col)?;
            }
        }
        Ok(())
    }

    /// Helper: Send Note On message
    fn send_note_on(&self, conn: &mut MidiOutputConnection, note: u8, velocity: u8) -> Result<(), String> {

        // Note On: 0x90 + channel (channel 0 = 0x90)
        conn.send(&[0x90, note, velocity])
            .map_err(|e| format!("Failed to send Note On: {}", e))?;

        Ok(())
    }

    /// Helper: Convert button control to note (if it has one)
    /// Some buttons may have note mappings for RGB control
    fn button_to_note(&self, _control: Push2Control) -> Option<u8> {
        // Most buttons don't have note mappings for RGB
        // This would need to be extended based on specific button RGB support
        None
    }
}

impl Default for LedController {
    fn default() -> Self {
        Self::new()
    }
}

