use fundsp::Float;
use fundsp::hacker::Atomic;
use fundsp::prelude::midi_hz;
use fundsp::shared::Shared;
use midi_msg::{ChannelVoiceMsg, ControlChange, MidiMsg};
use read_input::prelude::input;
use crate::synth::pitch_bend_factor;
use crate::ui_state::{OpPage, Page, UIState};
use crate::voice_params::{OpParams, VoiceParams};

pub fn encoder_to_value(input: u8, value: f64, intensity: f64) -> f64 {
	if input > 64 { value + (-128 + input as i8) as f64 / intensity }
	else { value + input as f64 / intensity }
}

pub fn encoder_to_shared<F: Float + Atomic>(input: u8, value: &Shared<F>, intensity: F) {
	value.set_value(
		F::from_f64(encoder_to_value(input, value.value().to_f64(), intensity.to_f64()))
	)
}

pub fn control_to_pages(control: ControlChange, ui: &UIState) {
	match control {
		ControlChange::UndefinedHighRes {control1, control2: _, value} => {
			if value > 0 {
				match control1 {
					20 => { let mut page = ui.page.lock().unwrap(); *page = Page::Op1 }
					21 => { let mut page = ui.page.lock().unwrap(); *page = Page::Op2 }
					22 => { let mut page = ui.page.lock().unwrap(); *page = Page::Op3 }
					23 => { let mut page = ui.page.lock().unwrap(); *page = Page::Op4 }
					_ => {}
				}
			}
		}
		ControlChange::Undefined {control, value} => {
			if value > 0 {
				match control {
					102 => { let mut page = ui.op_subpage.lock().unwrap(); *page = OpPage::Tone }
					103 => { let mut page = ui.op_subpage.lock().unwrap(); *page = OpPage::Amp }
					_ => {}
				}
			}
		}
		_ => {}
	}
}

pub enum Pot {
	MainPot(u8, u8),
}

pub fn pots_to_sub_page(pot: &Pot, op_subpage: OpPage, op_params: &OpParams) {
	match op_subpage {
		OpPage::Tone => {
			if let Pot::MainPot(id, value) = pot {
				match id { 
					1 => { encoder_to_shared(*value, &op_params.volume, 16.) }
					2 => { encoder_to_shared(*value, &op_params.ratio, 1.) }
					_ => {}
				}
			}
		}
		OpPage::Amp => {
			if let Pot::MainPot(id, value) = pot {
				match id {
					1 => { encoder_to_shared(*value, &op_params.adsr_params.a, 32.) }
					2 => { encoder_to_shared(*value, &op_params.adsr_params.d, 32.) }
					3 => { encoder_to_shared(*value, &op_params.adsr_params.s, 32.) }
					4 => { encoder_to_shared(*value, &op_params.adsr_params.r, 32.) }
					_ => {}
				}
			}
		}
	}
}

pub fn pots_to_controls(control: ControlChange, voice_params: &VoiceParams, ui: &UIState) {
	let mut page = ui.page.lock().unwrap();
	
	let pot = match control {
		ControlChange::SoundControl2(x) => { Some(Pot::MainPot(1, x)) }
		ControlChange::SoundControl3(x) => { Some(Pot::MainPot(2, x)) }
		ControlChange::SoundControl4(x) => { Some(Pot::MainPot(3, x)) }
		ControlChange::SoundControl5(x) => { Some(Pot::MainPot(4, x)) }
		ControlChange::SoundControl6(x) => { Some(Pot::MainPot(5, x)) }
		ControlChange::SoundControl7(x) => { Some(Pot::MainPot(6, x)) }
		ControlChange::SoundControl8(x) => { Some(Pot::MainPot(7, x)) }
		ControlChange::SoundControl9(x) => { Some(Pot::MainPot(8, x)) }
		_ => { None }
	};

	if let Some(pot) = pot {
		match *page {
			Page::Op1 => {
				let op_subpage = ui.op_subpage.lock().unwrap();
				pots_to_sub_page(&pot, op_subpage.to_owned(), &voice_params.op1)
			}
			Page::Op2 => {
				let op_subpage = ui.op_subpage.lock().unwrap();
				pots_to_sub_page(&pot, op_subpage.to_owned(), &voice_params.op2)
			}
			
			_ => {}
		}
	}
	
}

pub fn midi_to_params(midi_msg: MidiMsg, voice_params: &VoiceParams, ui: &UIState) {
	match midi_msg {
		MidiMsg::ChannelVoice { channel, msg } => {
			println!("Received {channel} {msg:?}");
			match msg {
				ChannelVoiceMsg::NoteOn { note, velocity } => {
					if note > 12 {     // filter encoder touches on push 2
						voice_params.pitch.set_value(midi_hz(note as f64));
						voice_params.volume.set_value(velocity as f64 / 127.0);
						voice_params.pitch_bend.set_value(1.0);
						voice_params.control.set_value(1.0);
					}
				}
				ChannelVoiceMsg::NoteOff { note, velocity: _ } => {
					if voice_params.pitch.value() == midi_hz(note as f64) {
						voice_params.control.set_value(-1.0);
					}
				}
				ChannelVoiceMsg::PitchBend { bend } => {
					voice_params.pitch_bend.set_value(pitch_bend_factor(bend));
				}
				ChannelVoiceMsg::ControlChange { control } => {
					control_to_pages(control, ui);
					pots_to_controls(control, voice_params, ui)
				}
				_ => {}
			}
		}
		_ => {}
	}
}