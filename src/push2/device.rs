/// Push 2 device connection and management
///
/// This module provides the main interface for connecting to and
/// interacting with the Ableton Push 2 device.
use crate::push2::controls::Push2Control;
use crate::push2::events::{EventQueue, Push2Event, Push2EventHandler};
use crate::push2::led::LedController;
use crate::push2::midi::{
    encoder_touch_note_to_id, encoder_value_to_delta, is_encoder_touch_note, is_pad_note,
};
use anyhow::{bail, Result};
use midi_msg::{ChannelVoiceMsg, ControlChange, MidiMsg};
use midir::{
    Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput, MidiOutputConnection,
    MidiOutputPort,
};
use std::sync::mpsc::{channel, Receiver, Sender};

/// Builder for Push2Device
pub struct Push2DeviceBuilder {
    use_user_port: bool,
    event_queue_size: usize,
    velocity_sensitive: bool,
}

impl Push2DeviceBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            use_user_port: false, // Default to Live port
            event_queue_size: 1000,
            velocity_sensitive: true,
        }
    }

    /// Use User port instead of Live port
    pub fn use_user_port(mut self, use_user: bool) -> Self {
        self.use_user_port = use_user;
        self
    }

    /// Set event queue size
    pub fn event_queue_size(mut self, size: usize) -> Self {
        self.event_queue_size = size;
        self
    }

    /// Enable/disable velocity sensitivity
    pub fn velocity_sensitive(mut self, enabled: bool) -> Self {
        self.velocity_sensitive = enabled;
        self
    }

    /// Build the Push2Device
    pub fn build(self) -> Result<Push2Device> {
        Push2Device::new(self.use_user_port, self.event_queue_size)
    }
}

impl Default for Push2DeviceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Main Push 2 device interface
pub struct Push2Device {
    midi_input: MidiInput,
    midi_output: MidiOutput,
    input_port: MidiInputPort,
    output_port: MidiOutputPort,
    input_connection: Option<MidiInputConnection<()>>,
    output_connection: Option<MidiOutputConnection>,
    event_queue: EventQueue,
    event_tx: Sender<Push2Event>,
    event_rx: Receiver<Push2Event>,
    led_controller: LedController,
}

impl Push2Device {
    /// Create a new Push2Device
    fn new(use_user_port: bool, queue_size: usize) -> Result<Self> {
        let mut midi_input = MidiInput::new("Push2 Input")?;
        let midi_output = MidiOutput::new("Push2 Output")?;

        midi_input.ignore(Ignore::None);

        // Find Push 2 ports
        let input_port = Self::find_input_port(&midi_input, use_user_port)?;
        let output_port = Self::find_output_port(&midi_output, use_user_port)?;

        let (event_tx, event_rx) = channel();
        let event_queue = EventQueue::new(queue_size);
        let led_controller = LedController::new();

        Ok(Self {
            midi_input,
            midi_output,
            input_port,
            output_port,
            input_connection: None,
            output_connection: None,
            event_queue,
            event_tx,
            event_rx,
            led_controller,
        })
    }

    /// Find the appropriate input port
    fn find_input_port(midi_input: &MidiInput, use_user_port: bool) -> Result<MidiInputPort> {
        let ports = midi_input.ports();
        if ports.is_empty() {
            bail!("No MIDI input devices found");
        }

        let port_names: Vec<(MidiInputPort, String)> = ports
            .iter()
            .map(|port| {
                let name = midi_input.port_name(port).unwrap_or_default();
                (port.clone(), name)
            })
            .collect();

        // Look for Push 2 ports
        for (port, name) in &port_names {
            let is_user = name.contains("User") || name.contains("MIDIIN2");
            let is_live = name.contains("Live") && !is_user;
            let is_push2 = name.contains("Push 2") || name.contains("Push2");

            if is_push2 {
                if use_user_port && is_user {
                    return Ok(port.clone());
                } else if !use_user_port && is_live {
                    return Ok(port.clone());
                }
            }
        }

        // Fallback: find any Push 2 port
        for (port, name) in &port_names {
            if name.contains("Push 2") || name.contains("Push2") {
                return Ok(port.clone());
            }
        }

        bail!("Ableton Push 2 input port not found");
    }

    /// Find the appropriate output port
    fn find_output_port(midi_output: &MidiOutput, use_user_port: bool) -> Result<MidiOutputPort> {
        let ports = midi_output.ports();
        if ports.is_empty() {
            bail!("No MIDI output devices found");
        }

        let port_names: Vec<(MidiOutputPort, String)> = ports
            .iter()
            .map(|port| {
                let name = midi_output.port_name(port).unwrap_or_default();
                (port.clone(), name)
            })
            .collect();

        // Look for Push 2 ports
        for (port, name) in &port_names {
            let is_user = name.contains("User") || name.contains("MIDIOUT2");
            let is_live = name.contains("Live") && !is_user;
            let is_push2 = name.contains("Push 2") || name.contains("Push2");

            if is_push2 {
                if use_user_port && is_user {
                    return Ok(port.clone());
                } else if !use_user_port && is_live {
                    return Ok(port.clone());
                }
            }
        }

        // Fallback: find any Push 2 port
        for (port, name) in &port_names {
            if name.contains("Push 2") || name.contains("Push2") {
                return Ok(port.clone());
            }
        }

        bail!("Ableton Push 2 output port not found");
    }

    /// Connect to the Push 2 device
    pub fn connect(&mut self) -> Result<()> {
        if self.input_connection.is_some() {
            bail!("Already connected");
        }

        let input_port_name = self.midi_input.port_name(&self.input_port)?;
        let output_port_name = self.midi_output.port_name(&self.output_port)?;

        let event_tx = self.event_tx.clone();
        let input_port = self.input_port.clone();
        let output_port = self.output_port.clone();

        // Take ownership of MIDI objects for connection
        let midi_input = std::mem::replace(&mut self.midi_input, MidiInput::new("Push2 Input")?);
        let midi_output =
            std::mem::replace(&mut self.midi_output, MidiOutput::new("Push2 Output")?);

        // Connect input
        let input_conn = midi_input
            .connect(
                &input_port,
                "push2-input",
                move |_stamp, message, _| {
                    if let Ok((msg, _len)) = MidiMsg::from_midi(message) {
                        if let Some(event) = Self::midi_to_event(&msg) {
                            // Send to channel for real-time processing
                            // Queue is available separately if needed for buffering
                            let _ = event_tx.send(event);
                        }
                    }
                },
                (),
            )
            .map_err(|e| anyhow::anyhow!("Failed to connect MIDI input: {}", e))?;

        // Connect output
        let output_conn = midi_output
            .connect(&output_port, "push2-output")
            .map_err(|e| anyhow::anyhow!("Failed to connect MIDI output: {}", e))?;

        self.input_connection = Some(input_conn);
        self.output_connection = Some(output_conn);

        println!(
            "Connected to Push 2 (Input: {}, Output: {})",
            input_port_name, output_port_name
        );

        Ok(())
    }

    /// Disconnect from the Push 2 device
    pub fn disconnect(&mut self) {
        self.input_connection = None;
        self.output_connection = None;
        println!("Disconnected from Push 2");
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.input_connection.is_some()
    }

    /// Convert MIDI message to Push2Event
    fn midi_to_event(msg: &MidiMsg) -> Option<Push2Event> {
        match msg {
            MidiMsg::ChannelVoice { channel: _, msg } => {
                match msg {
                    ChannelVoiceMsg::NoteOn { note, velocity } => {
                        // Check for encoder touches first (notes 0-10)
                        if is_encoder_touch_note(*note) {
                            if let Some(encoder_id) = encoder_touch_note_to_id(*note) {
                                return Some(Push2Event::EncoderTouched {
                                    encoder_id,
                                    velocity: *velocity,
                                });
                            }
                        }

                        // Check for pad notes (notes 36-99)
                        if !is_pad_note(*note) {
                            return None;
                        }

                        if let Some((row, col)) = Push2Control::note_to_pad_coords(*note) {
                            Some(Push2Event::PadPressed {
                                row,
                                col,
                                velocity: *velocity,
                                aftertouch: None,
                            })
                        } else {
                            None
                        }
                    }
                    ChannelVoiceMsg::NoteOff { note, velocity: _ } => {
                        // Check for encoder releases first (notes 0-10)
                        if is_encoder_touch_note(*note) {
                            if let Some(encoder_id) = encoder_touch_note_to_id(*note) {
                                return Some(Push2Event::EncoderReleased { encoder_id });
                            }
                        }

                        // Check for pad notes (notes 36-99)
                        if !is_pad_note(*note) {
                            return None;
                        }

                        if let Some((row, col)) = Push2Control::note_to_pad_coords(*note) {
                            Some(Push2Event::PadReleased { row, col })
                        } else {
                            None
                        }
                    }
                    ChannelVoiceMsg::ControlChange { control } => {
                        Self::control_change_to_event(control)
                    }
                    ChannelVoiceMsg::PitchBend { bend } => {
                        Some(Push2Event::PitchBend { bend: *bend as i16 })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Convert ControlChange to Push2Event
    fn control_change_to_event(control: &ControlChange) -> Option<Push2Event> {
        match control {
            ControlChange::SoundControl2(value) => {
                // Track encoder 1 (CC 71)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 1,
                    delta,
                })
            }
            ControlChange::SoundControl3(value) => {
                // Track encoder 2 (CC 72)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 2,
                    delta,
                })
            }
            ControlChange::SoundControl4(value) => {
                // Track encoder 3 (CC 73)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 3,
                    delta,
                })
            }
            ControlChange::SoundControl5(value) => {
                // Track encoder 4 (CC 74)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 4,
                    delta,
                })
            }
            ControlChange::SoundControl6(value) => {
                // Track encoder 5 (CC 75)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 5,
                    delta,
                })
            }
            ControlChange::SoundControl7(value) => {
                // Track encoder 6 (CC 76)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 6,
                    delta,
                })
            }
            ControlChange::SoundControl8(value) => {
                // Track encoder 7 (CC 77)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 7,
                    delta,
                })
            }
            ControlChange::SoundControl9(value) => {
                // Track encoder 8 (CC 78)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 8,
                    delta,
                })
            }
            ControlChange::SoundControl10(value) => {
                // Master encoder (CC 79)
                let delta = encoder_value_to_delta(*value);
                Some(Push2Event::EncoderTurned {
                    encoder_id: 9,
                    delta,
                })
            }
            ControlChange::CC { control, value } => {
                // Check if it's an encoder (tempo/swing)
                if *control == 14 {
                    // Tempo encoder
                    let delta = encoder_value_to_delta(*value);
                    return Some(Push2Event::EncoderTurned {
                        encoder_id: 10,
                        delta,
                    });
                }
                if *control == 15 {
                    // Swing encoder
                    let delta = encoder_value_to_delta(*value);
                    return Some(Push2Event::EncoderTurned {
                        encoder_id: 11,
                        delta,
                    });
                }

                // Check if it's a button
                if let Some(button) = Push2Control::from_cc(*control) {
                    if *value > 0 {
                        Some(Push2Event::ButtonPressed { control: button })
                    } else {
                        Some(Push2Event::ButtonReleased { control: button })
                    }
                } else if *control == 1 {
                    // Touch strip
                    Some(Push2Event::TouchStripChanged { value: *value })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Process events with a handler (blocking)
    pub fn process_events<H: Push2EventHandler>(&self, handler: &mut H) -> Result<()> {
        // Process from channel only
        // The queue is available separately via event_queue() if buffering is needed
        while let Ok(event) = self.event_rx.try_recv() {
            handler.handle_event(&event);
        }

        Ok(())
    }

    /// Process events non-blocking
    pub fn process_events_non_blocking<H: Push2EventHandler>(&self, handler: &mut H) {
        // Process from channel only (non-blocking)
        // The queue is available separately via event_queue() if buffering is needed
        while let Ok(event) = self.event_rx.try_recv() {
            handler.handle_event(&event);
        }
    }

    /// Get reference to event queue
    pub fn event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    /// Get event sender (for external use)
    pub fn event_sender(&self) -> &Sender<Push2Event> {
        &self.event_tx
    }
    
    /// Get LED controller
    pub fn led_controller(&self) -> &LedController {
        &self.led_controller
    }
    
    /// Get mutable LED controller
    pub fn led_controller_mut(&mut self) -> &mut LedController {
        &mut self.led_controller
    }
    
    /// Get mutable reference to output connection for LED operations
    /// This allows LED controller methods to access the connection
    pub fn output_connection_mut(&mut self) -> Option<&mut MidiOutputConnection> {
        self.output_connection.as_mut()
    }
    
    /// Send color palette to Push 2
    pub fn send_palette(&mut self) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.send_palette(conn)
    }
    
    /// Set pad color and animation
    pub fn set_pad_color(
        &mut self,
        row: u8,
        col: u8,
        color_index: u8,
        animation: crate::push2::led::LedAnimation,
    ) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.set_pad_color(conn, row, col, color_index, animation)
    }
    
    /// Set pad color using RGB color (automatically adds to palette)
    pub fn set_pad_color_rgb(
        &mut self,
        row: u8,
        col: u8,
        color: crate::push2::led::RgbColor,
        animation: crate::push2::led::LedAnimation,
    ) -> Result<(), String> {
        // Get or add color to palette
        let color_index = self.led_controller.get_or_add_color(color)
            .ok_or("Palette is full")?;
        
        // Send palette if it was updated
        self.send_palette()?;
        
        // Set pad color
        self.set_pad_color(row, col, color_index, animation)
    }
    
    /// Clear a pad
    pub fn clear_pad(&mut self, row: u8, col: u8) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.clear_pad(conn, row, col)
    }
    
    /// Set button LED
    pub fn set_button_led(
        &mut self,
        control: Push2Control,
        color_index: u8,
        animation: crate::push2::led::LedAnimation,
    ) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.set_button_led(conn, control, color_index, animation)
    }
    
    /// Set button LED using RGB color (automatically adds to palette)
    pub fn set_button_led_rgb(
        &mut self,
        control: Push2Control,
        color: crate::push2::led::RgbColor,
        animation: crate::push2::led::LedAnimation,
    ) -> Result<(), String> {
        // Get or add color to palette
        let color_index = self.led_controller.get_or_add_color(color)
            .ok_or("Palette is full")?;
        
        // Send palette if it was updated
        self.send_palette()?;
        
        // Set button LED
        self.set_button_led(control, color_index, animation)
    }
    
    /// Clear a button
    pub fn clear_button(&mut self, control: Push2Control) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.clear_button(conn, control)
    }
    
    /// Set touch strip color
    pub fn set_touch_strip_color(&mut self, color_index: u8) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.set_touch_strip_color(conn, color_index)
    }
    
    /// Set touch strip color using RGB color (automatically adds to palette)
    pub fn set_touch_strip_color_rgb(
        &mut self,
        color: crate::push2::led::RgbColor,
    ) -> Result<(), String> {
        // Get or add color to palette
        let color_index = self.led_controller.get_or_add_color(color)
            .ok_or("Palette is full")?;
        
        // Send palette if it was updated
        self.send_palette()?;
        
        // Set touch strip color
        self.set_touch_strip_color(color_index)
    }
    
    /// Clear touch strip
    pub fn clear_touch_strip(&mut self) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.clear_touch_strip(conn)
    }
    
    /// Clear all pads
    pub fn clear_all_pads(&mut self) -> Result<(), String> {
        let conn = self.output_connection.as_mut()
            .ok_or("Not connected")?;
        self.led_controller.clear_all_pads(conn)
    }
}
