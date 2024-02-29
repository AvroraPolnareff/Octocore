mod r#voice_params;
mod graphix;
mod push;
mod synth;
mod ui_state;

use midir::{MidiInput};
use crate::graphix::render_image;
use crate::push::{Push2};
use crate::synth::{get_midi_device, run_input, run_output};
use crate::voice_params::VoiceParams;


fn main() -> anyhow::Result<()> {
  let mut midi_in = MidiInput::new("midir reading input")?;
  let in_port = get_midi_device(&mut midi_in)?;

  let voice_params= VoiceParams::default();
  
  let params = voice_params.clone();

  std::thread::spawn(move || {
    let mut push = Push2::new();
    push.connect();
    loop {
      //let now = Instant::now();
      let mut pixels: [u8; 2048 * 160] = [0; 2048 * 160];
      let mut pixels = render_image(&params, &mut pixels);
      push.draw_image(&mut pixels);
      //let elapsed = now.elapsed();
      //println!("Elapsed: {:.2?}", elapsed);
    }
  });

  run_output(
      voice_params.clone()
  );
  run_input(midi_in, in_port, voice_params)
}

