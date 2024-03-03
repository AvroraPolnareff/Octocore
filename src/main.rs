mod r#voice_params;
mod graphix;
mod push;
mod synth;
mod ui_state;
mod adsr;
mod midi_input;
mod midi_output;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel};
use midir::{MidiInput, MidiOutput};
use crate::graphix::render_image;
use crate::midi_input::{get_midi_device, run_input};
use crate::midi_output::{get_midi_out_device, run_midi_out};
use crate::push::{Push2};
use crate::synth::{run_output};
use crate::ui_state::{OpPage, Page, UIEvent, UIState};
use crate::voice_params::VoiceParams;


fn main() -> anyhow::Result<()> {
  let mut midi_in = MidiInput::new("midir reading input")?;
  let mut midi_out = MidiOutput::new("midir writing output")?;
  let in_port = get_midi_device(&mut midi_in)?;
  let out_port = get_midi_out_device(&mut midi_out)?;

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
  let (ui_tx, ui_rx) = channel::<UIEvent>();

  run_output(
      voice_params.clone()
  );
  run_midi_out(midi_out, &out_port, ui_rx);
  run_input(midi_in, in_port, voice_params, ui_state, ui_tx.clone())
}

