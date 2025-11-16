/// Push 2 Demo - Interactive demonstration of the Push 2 API
///
/// This example demonstrates:
/// - Connecting to Push 2 device
/// - Handling pad presses and releases
/// - Handling encoder turns
/// - Handling button presses
/// - Real-time event processing
/// - LED color control and visual feedback
///
/// Run with: cargo run --example push2_demo
use octocore::push2::{
    Push2Control, Push2Device, Push2DeviceBuilder, Push2EventHandler,
    RgbColor, LedAnimation, ColorPalette,
};
use std::time::{Duration, Instant};

struct Push2Demo {
    pad_press_count: usize,
    encoder_turn_count: usize,
    button_press_count: usize,
    last_event_time: Instant,
    start_time: Instant,
    pad_colors: [[Option<u8>; 8]; 8], // Track color index for each pad
    last_pad_pressed: Option<(u8, u8, u8)>, // (row, col, velocity)
    last_pad_released: Option<(u8, u8)>,
    last_encoder_turned: Option<(u8, i8)>,
    last_button_pressed: Option<Push2Control>,
    last_touch_strip_value: Option<u8>,
}

impl Push2Demo {
    fn new() -> Self {
        Self {
            pad_press_count: 0,
            encoder_turn_count: 0,
            button_press_count: 0,
            last_event_time: Instant::now(),
            start_time: Instant::now(),
            pad_colors: [[None; 8]; 8],
            last_pad_pressed: None,
            last_pad_released: None,
            last_encoder_turned: None,
            last_button_pressed: None,
            last_touch_strip_value: None,
        }
    }
    
    fn init_leds(device: &mut Push2Device) -> Result<(), String> {
        // Create a rainbow palette
        let palette = ColorPalette::rainbow(64);
        device.led_controller_mut().set_palette(palette);
        
        // Send palette to Push 2
        device.send_palette()?;
        
        // Clear all pads initially
        device.clear_all_pads()?;
        
        // Set some button LEDs to show they're active
        device.set_button_led_rgb(
            Push2Control::PlayButton,
            RgbColor::GREEN,
            LedAnimation::Static,
        )?;
        
        device.set_button_led_rgb(
            Push2Control::SessionButton,
            RgbColor::BLUE,
            LedAnimation::Static,
        )?;
        
        println!("✨ LED palette initialized with 64 rainbow colors");
        Ok(())
    }

    fn print_stats(&self) {
        let elapsed = self.start_time.elapsed().as_secs();
        let time_since_last = self.last_event_time.elapsed().as_secs();

        println!("\n╔════════════════════════════════════════╗");
        println!("║        Push 2 Event Statistics          ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ Pads pressed:     {:>18} ║", self.pad_press_count);
        println!("║ Encoders turned:  {:>18} ║", self.encoder_turn_count);
        println!("║ Buttons pressed:   {:>18} ║", self.button_press_count);
        println!("║ Runtime:           {:>15}s ║", elapsed);
        println!("║ Last event:        {:>15}s ago ║", time_since_last);
        println!("╚════════════════════════════════════════╝\n");
    }
}

impl Push2EventHandler for Push2Demo {
    fn handle_pad_pressed(&mut self, row: u8, col: u8, velocity: u8, aftertouch: Option<u8>) {
        self.pad_press_count += 1;
        self.last_event_time = Instant::now();
        self.last_pad_pressed = Some((row, col, velocity));

        let velocity_bar = "█".repeat((velocity as usize * 20) / 127);
        let aftertouch_str = if let Some(at) = aftertouch {
            format!(" (aftertouch: {})", at)
        } else {
            String::new()
        };

        println!(
            "🎹 Pad ({}, {}) pressed | Velocity: {:>3} {} {}",
            row, col, velocity, velocity_bar, aftertouch_str
        );
    }

    fn handle_pad_released(&mut self, row: u8, col: u8) {
        self.last_event_time = Instant::now();
        self.last_pad_released = Some((row, col));
        self.last_pad_pressed = None; // Clear pressed state
        println!("🎹 Pad ({}, {}) released", row, col);
    }

    fn handle_pad_aftertouch(&mut self, row: u8, col: u8, pressure: u8) {
        self.last_event_time = Instant::now();
        let pressure_bar = "█".repeat((pressure as usize * 20) / 127);
        println!(
            "🎹 Pad ({}, {}) aftertouch: {:>3} {}",
            row, col, pressure, pressure_bar
        );
    }

    fn handle_encoder_turned(&mut self, encoder_id: u8, delta: i8) {
        self.encoder_turn_count += 1;
        self.last_event_time = Instant::now();
        self.last_encoder_turned = Some((encoder_id, delta));

        let encoder_name = match encoder_id {
            1..=8 => format!("Track {}", encoder_id),
            9 => "Master".to_string(),
            10 => "Tempo".to_string(),
            11 => "Swing".to_string(),
            _ => format!("Encoder {}", encoder_id),
        };

        let direction = if delta > 0 { "→" } else { "←" };
        let delta_bar = "█".repeat(delta.abs() as usize);

        println!(
            "🎚️  {} encoder {} {:>3} {}",
            encoder_name, direction, delta, delta_bar
        );
    }

    fn handle_encoder_touched(&mut self, encoder_id: u8, velocity: u8) {
        self.last_event_time = Instant::now();

        let encoder_name = match encoder_id {
            1..=8 => format!("Track {}", encoder_id),
            9 => "Master".to_string(),
            10 => "Tempo".to_string(),
            11 => "Swing".to_string(),
            _ => format!("Encoder {}", encoder_id),
        };

        println!(
            "👆 {} encoder TOUCHED (velocity: {})",
            encoder_name, velocity
        );
    }

    fn handle_encoder_released(&mut self, encoder_id: u8) {
        self.last_event_time = Instant::now();

        let encoder_name = match encoder_id {
            1..=8 => format!("Track {}", encoder_id),
            9 => "Master".to_string(),
            10 => "Tempo".to_string(),
            11 => "Swing".to_string(),
            _ => format!("Encoder {}", encoder_id),
        };

        println!("👆 {} encoder RELEASED", encoder_name);
    }

    fn handle_button_pressed(&mut self, control: Push2Control) {
        self.button_press_count += 1;
        self.last_event_time = Instant::now();

        // Categorize buttons for better display
        let category = match control {
            Push2Control::PlayButton
            | Push2Control::RecordButton
            | Push2Control::NewButton
            | Push2Control::DuplicateButton
            | Push2Control::AutomateButton
            | Push2Control::FixedLengthButton => "Transport",

            Push2Control::TrackButton(_)
            | Push2Control::TrackButtonAbove(_)
            | Push2Control::MasterButton => "Track",

            Push2Control::SceneButton(_) => "Scene",

            Push2Control::ArrowLeft
            | Push2Control::ArrowRight
            | Push2Control::ArrowUp
            | Push2Control::ArrowDown
            | Push2Control::SelectButton => "Navigation",

            Push2Control::ShiftButton | Push2Control::NoteButton | Push2Control::SessionButton => {
                "Mode"
            }

            Push2Control::DeviceButton
            | Push2Control::BrowseButton
            | Push2Control::MixButton
            | Push2Control::ClipButton => "View",

            _ => "Other",
        };

        println!("🔘 [{}] {} pressed", category, control);
        self.last_button_pressed = Some(control.clone());
    }

    fn handle_button_released(&mut self, control: Push2Control) {
        self.last_event_time = Instant::now();
        println!("🔘 {} released", control);
    }

    fn handle_touch_strip_changed(&mut self, value: u8) {
        self.last_event_time = Instant::now();
        self.last_touch_strip_value = Some(value);
        let strip_bar = "█".repeat((value as usize * 40) / 127);
        println!("📊 Touch Strip: {:>3} {}", value, strip_bar);
    }

    fn handle_pitch_bend(&mut self, bend: i16) {
        self.last_event_time = Instant::now();
        let normalized = (bend as f32 / 8192.0) * 100.0;
        let center = 40;
        let pos = (center as f32 + normalized * 0.3) as usize;
        let bar = format!("{:>40}│", " ".repeat(pos.min(40)));
        println!("🎵 Pitch Bend: {:>6} {}", bend, bar);
    }
}

fn main() -> anyhow::Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║      Push 2 API Demo                  ║");
    println!("║      Press Ctrl+C to exit              ║");
    println!("╚════════════════════════════════════════╝\n");

    // Try to connect to Push 2
    println!("🔍 Looking for Push 2 device...");

    let mut device = Push2DeviceBuilder::new()
        .use_user_port(false)
        .event_queue_size(1000)
        .velocity_sensitive(true)
        .build()
        .unwrap();

    println!("🔌 Connecting to Push 2...");
    device.connect()?;
    println!("✅ Connected! Start interacting with your Push 2.\n");
    
    // Initialize LED palette and visual feedback
    println!("🎨 Initializing LED colors...");
    if let Err(e) = Push2Demo::init_leds(&mut device) {
        eprintln!("⚠️  Warning: Failed to initialize LEDs: {}", e);
        eprintln!("   Continuing without LED feedback...\n");
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 Tips:");
    println!("   • Press pads to see velocity-based colors");
    println!("   • Turn encoders to see row color changes");
    println!("   • Press buttons to see LED feedback");
    println!("   • Use touch strip to see color changes");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut demo = Push2Demo::new();
    let mut last_stats_time = Instant::now();

    // Main event loop
    loop {
        // Process events using standard handler
        device.process_events_non_blocking(&mut demo);
        
        // Apply LED feedback based on recent events
        if let Some((row, col, velocity)) = demo.last_pad_pressed.take() {
            // Visual feedback: Set pad color based on velocity
            let hue = ((row as f32 * 8.0 + col as f32) * 360.0 / 64.0) % 360.0;
            let saturation = 1.0;
            let brightness = (velocity as f32 / 127.0).max(0.3);
            let color = RgbColor::from_hsv(hue, saturation, brightness);
            
            if let Err(e) = device.set_pad_color_rgb(row, col, color, LedAnimation::Static) {
                eprintln!("⚠️  Failed to set pad color: {}", e);
            } else {
                if let Some(color_idx) = device.led_controller_mut().get_or_add_color(color) {
                    demo.pad_colors[row as usize][col as usize] = Some(color_idx);
                }
            }
        }
        
        if let Some((row, col)) = demo.last_pad_released.take() {
            // Dim the pad when released
            if let Some(color_idx) = demo.pad_colors[row as usize][col as usize] {
                if let Some(color) = device.led_controller_mut().palette().get_color(color_idx) {
                    let dimmed = RgbColor::new(
                        (color.r as f32 * 0.3) as u8,
                        (color.g as f32 * 0.3) as u8,
                        (color.b as f32 * 0.3) as u8,
                    );
                    let _ = device.set_pad_color_rgb(row, col, dimmed, LedAnimation::Static);
                }
            }
        }
        
        if let Some((encoder_id, delta)) = demo.last_encoder_turned.take() {
            // Visual feedback: Update pad row colors based on encoder
            if encoder_id >= 1 && encoder_id <= 8 {
                let row = (encoder_id - 1) as usize;
                if row < 8 {
                    let hue = ((demo.encoder_turn_count * delta.abs() as usize) % 360) as f32;
                    for col in 0..8 {
                        let saturation = 1.0 - (col as f32 / 8.0) * 0.3;
                        let color = RgbColor::from_hsv(hue, saturation, 0.8);
                        let _ = device.set_pad_color_rgb(row as u8, col as u8, color, LedAnimation::Static);
                    }
                }
            }
        }
        
        if let Some(control) = demo.last_button_pressed.take() {
            // Visual feedback for buttons
            match control {
                Push2Control::RecordButton => {
                    let _ = device.set_button_led_rgb(control, RgbColor::RED, LedAnimation::Blink);
                }
                Push2Control::PlayButton => {
                    let _ = device.set_button_led_rgb(control, RgbColor::GREEN, LedAnimation::Static);
                }
                Push2Control::SessionButton => {
                    let colors = [RgbColor::BLUE, RgbColor::CYAN, RgbColor::MAGENTA];
                    let color_idx = (demo.button_press_count % colors.len()) as usize;
                    let _ = device.set_button_led_rgb(control, colors[color_idx], LedAnimation::Static);
                }
                _ => {
                    let _ = device.set_button_led_rgb(control, RgbColor::YELLOW, LedAnimation::Static);
                }
            }
        }
        
        if let Some(value) = demo.last_touch_strip_value.take() {
            // Visual feedback: Change touch strip color based on value
            let hue = (value as f32 * 360.0 / 127.0) % 360.0;
            let color = RgbColor::from_hsv(hue, 1.0, 1.0);
            let _ = device.set_touch_strip_color_rgb(color);
        }

        // Print stats every 5 seconds
        if last_stats_time.elapsed() >= Duration::from_secs(5) {
            demo.print_stats();
            last_stats_time = Instant::now();
        }

        // Small sleep to prevent CPU spinning
        std::thread::sleep(Duration::from_millis(10));

        // Check if we should exit (this is a simple demo, Ctrl+C will work)
        // In a real app, you'd have proper signal handling
    }
}
