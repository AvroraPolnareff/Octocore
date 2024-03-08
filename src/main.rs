mod r#voice_params;
mod graphix;
mod push;
mod synth;
mod ui_state;
mod adsr;
mod midi_input;
mod midi_output;
mod poly;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel};
use fundsp::hacker32::{Net64};
use fundsp::hacker::{NodeId, pass, sum, U128};
use midir::{MidiInput, MidiOutput};
use crate::graphix::render_image;
use crate::midi_input::{get_midi_device, run_input};
use crate::midi_output::{get_midi_out_device, get_midi_out_connection, init_midi_ui, send_ui_midi};
use crate::poly::{MonoPoly};
use crate::push::{Push2};
use crate::synth::{create_sound, run_output, sine_lfo};
use crate::ui_state::{OpPage, Page, InputEvent, UIState};
use crate::voice_params::SynthParams;

fn render_loop(synth_params: SynthParams, uistate: UIState) {
  std::thread::spawn(move || {
    let mut push = Push2::new();
    let mut pixels: [u8; 2048 * 160] = [0; 2048 * 160];
    push.connect();
    loop {
      let mut pixels = render_image(&synth_params, uistate.clone(), &mut pixels);
      push.draw_image(&mut pixels);
    }
  });
}

fn main() -> anyhow::Result<()> {
  let mut midi_in = MidiInput::new("midir reading input")?;
  let mut midi_out = MidiOutput::new("midir writing output")?;
  let in_port = get_midi_device(&mut midi_in)?;
  let out_port = get_midi_out_device(&mut midi_out)?;
  let mut mono_poly = MonoPoly::new(8);
  let synth_params = SynthParams::new(mono_poly.voice_size);

  let ui_state = UIState {
    page: Arc::new(Mutex::new(Page::Op1)),
    op_subpage: Arc::new(Mutex::new(OpPage::Tone))
  };

  render_loop(synth_params.clone(), ui_state.clone());
  
  let (ui_tx, ui_rx) = channel::<InputEvent>();
  
  let mut net = Net64::new(0, 1);
  let dummy_id = net.push(Box::new(sum::<U128, _, _>(|_| pass())));
  net.connect_output(dummy_id, 0, 0);
  
  let voice_ids: Vec<(usize, NodeId)> = (0 .. mono_poly.voice_size)
    .map(|i| net.push(create_sound(&synth_params, i)))
    .enumerate().collect();
  for (i, id) in voice_ids {
    net.connect(id, 0, dummy_id, i)
  }

  let mut connection = get_midi_out_connection(midi_out, &out_port);
  run_output(net.backend());
  {
    let synth_params = synth_params.clone();
    std::thread::spawn(move || {
      init_midi_ui(&mut connection);
      for event in ui_rx {
        send_ui_midi(&event, &mut connection);
        match event {
          InputEvent::PageChange(_) => {}
          InputEvent::OpSubpageChange(_) => {}
          InputEvent::LFO(x) => {
            if x > 0. {
              net.replace(dummy_id, sine_lfo());
            } else {
              net.replace(dummy_id, Box::new(pass()));
            }
            net.commit();
          }
          InputEvent::NoteOn {note, velocity} => {
            mono_poly.on_voice_on(note, velocity, &synth_params.voice_params)
          }
          InputEvent::NoteOff {note} => {
            mono_poly.on_voice_off(note, &synth_params.voice_params)
          }
        }
      }
    });
  }
  
  run_input(midi_in, in_port, synth_params, ui_state, ui_tx.clone())
}

