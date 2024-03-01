mod r#voice_params;
mod graphix;
mod push;
mod synth;
mod ui_state;
mod adsr;
mod midi_input;

use std::sync::{Arc, Mutex};
use midir::{MidiInput};
use crate::graphix::render_image;
use crate::push::{Push2};
use crate::synth::{get_midi_device, run_input, run_output};
use crate::ui_state::{OpPage, Page, UIState};
use crate::voice_params::VoiceParams;


fn main() -> anyhow::Result<()> {
  let mut midi_in = MidiInput::new("midir reading input")?;
  let in_port = get_midi_device(&mut midi_in)?;

  let voice_params = VoiceParams::default();
  let ui_state = UIState {
    page: Arc::new(Mutex::new(Page::Op1)),
    op_subpage: Arc::new(Mutex::new(OpPage::Tone))
  };
  
  let cloned_params = voice_params.clone();
  let cloned_state = ui_state.clone();
  std::thread::spawn(move || {
    let mut push = Push2::new();
    push.connect();
    loop {
      //let now = Instant::now();
      let mut pixels: [u8; 2048 * 160] = [0; 2048 * 160];
      let mut pixels = render_image(&cloned_params, &cloned_state, &mut pixels);
      push.draw_image(&mut pixels);
      //let elapsed = now.elapsed();
      //println!("Elapsed: {:.2?}", elapsed);
    }
  });

  run_output(
      voice_params.clone()
  );
  run_input(midi_in, in_port, voice_params, ui_state)
}

