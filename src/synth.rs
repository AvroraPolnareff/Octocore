use anyhow::bail;
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::audionode::{Pipe, Stack};
use fundsp::audiounit::AudioUnit64;
use fundsp::combinator::An;
use fundsp::envelope::EnvelopeIn;
use fundsp::hacker::{midi_hz, clamp01, delerp, Frame, lfo_in, sine, var, U4, adsr_live, Shared};
use fundsp::hacker32::Var;
use fundsp::prelude::U5;
use midi_msg::{ChannelVoiceMsg, ControlChange, MidiMsg};
use midir::{Ignore, MidiInput, MidiInputPort};
use read_input::prelude::*;
use crate::adsr::adsr;

use crate::ui_state::{OpPage, Page, UIState};
use crate::voice_params::{AdsrParams, VoiceParams};


pub fn c_adsr(
	adsr_params: &AdsrParams,
	control: &Shared<f64>
) -> An<Pipe<
	f64, Stack<
		f64,
		Stack<f64, Stack<f64, Stack<f64, Var<f64>, Var<f64>>, Var<f64>>, Var<f64>>,
		Var<f64>
	>,
	EnvelopeIn<
		f64, f64, impl Fn(f64, &Frame<f64, U5>) -> f64 + Sized + Clone + Sized, U5, f64>
>> {
	(
		var(&adsr_params.a)
			| var(&adsr_params.d)
			| var(&adsr_params.s)
			| var(&adsr_params.r)
			| var(&control)
	)>> adsr()
}

pub fn create_sound(
	voice_params: &VoiceParams
) -> Box<dyn AudioUnit64> {
	
	let bf = || var(&voice_params.pitch) * var(&voice_params.pitch_bend);
	let ratio = 2.0;
	let modulator = bf() * ratio
		>> sine() * c_adsr(&voice_params.op2.adsr_params, &voice_params.control)
		* var(&voice_params.op2.volume);
	let base_tone =
		modulator * bf() + bf() >> sine();

	Box::new(
		base_tone * c_adsr(&voice_params.op1.adsr_params, &voice_params.control) * var(&voice_params.volume)
	)
}

pub fn get_midi_device(midi_in: &mut MidiInput) -> anyhow::Result<MidiInputPort> {
	midi_in.ignore(Ignore::None);
	let in_ports = midi_in.ports();
	if in_ports.is_empty() {
		bail!("No MIDI devices attached")
	} else {
		for (i,port) in midi_in.ports().iter().enumerate() {
			println!("{i}. {}", midi_in.port_name(port).unwrap())
		}
		let port = input::<usize>().msg("Select input port: ").get();
		println!(
			"Chose MIDI device {}",
			midi_in.port_name(&in_ports[port]).unwrap()
		);
		Ok(in_ports[port].clone())
	}
}

pub fn encoder_to_value(input: u8, value: f64, intensity: f64) -> f64 {
	if input > 64 { value + (-128 + input as i8) as f64 / intensity }
	else { value + input as f64 / intensity }
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



pub fn pots_to_controls(control: ControlChange, voice_params: &VoiceParams, ui: &UIState) {
	match control {
		ControlChange::SoundControl2(x) => {
			let mut page = ui.page.lock().unwrap();
			let mut op_subpage = ui.op_subpage.lock().unwrap();
			match *page {
				Page::Op1 => {
					match *op_subpage { 
						OpPage::Tone => {
							voice_params.op1.volume.set_value(
								encoder_to_value(x, voice_params.op1.volume.value(), 32.0)
							)
						}
						OpPage::Amp => {
							voice_params.op1.adsr_params.a.set_value(
								encoder_to_value(x, voice_params.op1.adsr_params.a.value(), 64.0)
							)
						}
					}
				}
				Page::Op2 => {
					match *op_subpage {
						OpPage::Tone => {
							voice_params.op2.volume.set_value(
								encoder_to_value(x, voice_params.op2.volume.value(), 32.0)
							)
						}
						OpPage::Amp => {
							voice_params.op2.adsr_params.a.set_value(
								encoder_to_value(x, voice_params.op2.adsr_params.a.value(), 64.0)
							)
						}
					}
				}
				_ => {}
			}
		}
		_ => {}
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

pub fn run_input(
	midi_in: MidiInput,
	in_port: MidiInputPort,
	voice_params: VoiceParams,
	ui: UIState
) -> anyhow::Result<()> {
	println!("\nOpening connection");
	let in_port_name = midi_in.port_name(&in_port)?;
	let _conn_in = midi_in
		.connect(
			&in_port,
			"midir-read-input",
			move |_stamp, message, _| {
				let (msg, _len) = MidiMsg::from_midi(message).unwrap();
				midi_to_params(msg, &voice_params, &ui)
			},
			(),
		)
		.unwrap();
	println!("Connection open, reading input from '{in_port_name}'");

	let _ = input::<String>().msg("(press enter to exit)...\n").get();
	println!("Closing connection");
	Ok(())
}

pub fn pitch_bend_factor(bend: u16) -> f64 {
	2.0_f64.powf(((bend as f64 - 8192.0) / 8192.0) / 12.0)
}

pub fn run_output(
	voice_params: VoiceParams,
) {
	let host = cpal::default_host();
	let device = host
		.default_output_device()
		.expect("failed to find a default output device");
	let config = device.default_output_config().unwrap();
	match config.sample_format() {
		SampleFormat::F32 => {
			run_synth::<f32>(voice_params, device, config.into())
		}
		SampleFormat::I16 => {
			run_synth::<i16>(voice_params, device, config.into())
		}
		SampleFormat::U16 => {
			run_synth::<u16>(voice_params, device, config.into())
		}
		_ => panic!("Unsupported format"),
	}
}

fn run_synth<T: SizedSample + FromSample<f64>>(
	voice_params: VoiceParams,
	device: Device,
	config: StreamConfig,
) {
	std::thread::spawn(move || {
		let sample_rate = config.sample_rate.0 as f64;
		let mut sound = create_sound(&voice_params);
		sound.set_sample_rate(sample_rate);

		let mut next_value = move || sound.get_stereo();
		let channels = config.channels as usize;
		let err_fn = |err| eprintln!("an error occurred on stream: {err}");
		let stream = device
			.build_output_stream(
				&config,
				move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
					write_data(data, channels, &mut next_value)
				},
				err_fn,
				None,
			)
			.unwrap();

		stream.play().unwrap();
		loop {
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});
}

pub fn write_data<T: SizedSample + FromSample<f64>>(
	output: &mut [T],
	channels: usize,
	next_sample: &mut dyn FnMut() -> (f64, f64),
) {
	for frame in output.chunks_mut(channels) {
		let sample = next_sample();
		let left: T = T::from_sample(sample.0);
		let right: T = T::from_sample(sample.1);

		for (channel, sample) in frame.iter_mut().enumerate() {
			*sample = if channel & 1 == 0 { left } else { right };
		}
	}
}

