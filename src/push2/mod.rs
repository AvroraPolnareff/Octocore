/// Push 2 API for Ableton Push 2 controller
///
/// This module provides a complete API for interacting with the
/// Ableton Push 2 hardware controller, including:
/// - Control mapping and event handling
/// - MIDI communication
/// - LED control (to be implemented in Phase 2)
/// - Display interface (to be implemented in Phase 3)
pub mod controls;
pub mod device;
pub mod events;
pub mod midi;
pub mod led;

// Re-export main types
pub use controls::Push2Control;
pub use device::{Push2Device, Push2DeviceBuilder};
pub use events::{EventQueue, EventQueueError, Push2Event, Push2EventHandler};
pub use led::*;