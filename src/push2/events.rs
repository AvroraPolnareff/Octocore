/// Event system for Push 2
///
/// This module provides structured events and handler traits for
/// processing Push 2 input events.
use crate::push2::controls::Push2Control;

/// All possible events from Push 2
#[derive(Debug, Clone)]
pub enum Push2Event {
    /// Pad pressed (8x8 grid, coordinates 0-7)
    PadPressed {
        row: u8,
        col: u8,
        velocity: u8,
        aftertouch: Option<u8>,
    },

    /// Pad released
    PadReleased { row: u8, col: u8 },

    /// Pad aftertouch changed
    PadAftertouch { row: u8, col: u8, pressure: u8 },

    /// Encoder turned (relative values)
    EncoderTurned {
        encoder_id: u8, // 1-8 for tracks, 9 for master, 10 for tempo, 11 for swing
        delta: i8,      // -63 to +63
    },

    /// Encoder touched (pressed down)
    EncoderTouched {
        encoder_id: u8, // 1-8 for tracks, 9 for master, 10 for tempo, 11 for swing
        velocity: u8,   // Touch velocity (usually 127)
    },

    /// Encoder released (touch released)
    EncoderReleased {
        encoder_id: u8, // 1-8 for tracks, 9 for master, 10 for tempo, 11 for swing
    },

    /// Button pressed
    ButtonPressed { control: Push2Control },

    /// Button released
    ButtonReleased { control: Push2Control },

    /// Touch strip value changed
    TouchStripChanged {
        value: u8, // 0-127
    },

    /// Pitch bend (if used)
    PitchBend {
        bend: i16, // -8192 to +8191
    },
}

/// Trait for handling Push 2 events
///
/// Users implement this trait to define their event handling logic.
/// All methods have default empty implementations, so you only need
/// to override the ones you care about.
pub trait Push2EventHandler {
    /// Pad pressed
    fn handle_pad_pressed(&mut self, row: u8, col: u8, velocity: u8, aftertouch: Option<u8>) {
        let _ = (row, col, velocity, aftertouch);
    }

    /// Pad released
    fn handle_pad_released(&mut self, row: u8, col: u8) {
        let _ = (row, col);
    }

    /// Pad aftertouch changed
    fn handle_pad_aftertouch(&mut self, row: u8, col: u8, pressure: u8) {
        let _ = (row, col, pressure);
    }

    /// Encoder turned
    fn handle_encoder_turned(&mut self, encoder_id: u8, delta: i8) {
        let _ = (encoder_id, delta);
    }

    /// Encoder touched (pressed down)
    fn handle_encoder_touched(&mut self, encoder_id: u8, velocity: u8) {
        let _ = (encoder_id, velocity);
    }

    /// Encoder released (touch released)
    fn handle_encoder_released(&mut self, encoder_id: u8) {
        let _ = encoder_id;
    }

    /// Button pressed
    fn handle_button_pressed(&mut self, control: Push2Control) {
        let _ = control;
    }

    /// Button released
    fn handle_button_released(&mut self, control: Push2Control) {
        let _ = control;
    }

    /// Touch strip value changed
    fn handle_touch_strip_changed(&mut self, value: u8) {
        let _ = value;
    }

    /// Pitch bend
    fn handle_pitch_bend(&mut self, bend: i16) {
        let _ = bend;
    }

    /// Handle any event (fallback method)
    ///
    /// This is called for all events and dispatches to the specific
    /// handler methods. Override this if you want custom event routing.
    fn handle_event(&mut self, event: &Push2Event) {
        match event {
            Push2Event::PadPressed {
                row,
                col,
                velocity,
                aftertouch,
            } => {
                self.handle_pad_pressed(*row, *col, *velocity, *aftertouch);
            }
            Push2Event::PadReleased { row, col } => {
                self.handle_pad_released(*row, *col);
            }
            Push2Event::PadAftertouch { row, col, pressure } => {
                self.handle_pad_aftertouch(*row, *col, *pressure);
            }
            Push2Event::EncoderTurned { encoder_id, delta } => {
                self.handle_encoder_turned(*encoder_id, *delta);
            }
            Push2Event::EncoderTouched {
                encoder_id,
                velocity,
            } => {
                self.handle_encoder_touched(*encoder_id, *velocity);
            }
            Push2Event::EncoderReleased { encoder_id } => {
                self.handle_encoder_released(*encoder_id);
            }
            Push2Event::ButtonPressed { control } => {
                self.handle_button_pressed(control.clone());
            }
            Push2Event::ButtonReleased { control } => {
                self.handle_button_released(control.clone());
            }
            Push2Event::TouchStripChanged { value } => {
                self.handle_touch_strip_changed(*value);
            }
            Push2Event::PitchBend { bend } => {
                self.handle_pitch_bend(*bend);
            }
        }
    }
}

/// Event queue for buffering events
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum EventQueueError {
    QueueFull,
}

/// Thread-safe event queue
#[derive(Clone)]
pub struct EventQueue {
    queue: Arc<Mutex<VecDeque<Push2Event>>>,
    max_size: usize,
}

impl EventQueue {
    /// Create a new event queue with specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
        }
    }

    /// Push an event to the queue
    pub fn push(&self, event: Push2Event) -> Result<(), EventQueueError> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= self.max_size {
            return Err(EventQueueError::QueueFull);
        }
        queue.push_back(event);
        Ok(())
    }

    /// Pop an event from the queue
    pub fn pop(&self) -> Option<Push2Event> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    /// Drain all events and process them with a handler
    pub fn drain<H: Push2EventHandler>(&self, handler: &mut H) {
        while let Some(event) = self.pop() {
            handler.handle_event(&event);
        }
    }

    /// Get current queue length
    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    /// Clear all events from the queue
    pub fn clear(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
    }
}
