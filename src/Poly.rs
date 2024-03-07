use fundsp::hacker32::midi_hz;
use crate::voice_params::VoiceParams;

pub type VoiceIndex = u8;

#[derive(Clone)]
pub struct Voice {
	pub note: u8,
	pub voice_index: VoiceIndex,
}
pub enum VoiceMode {
	OpenPoly
}
pub struct MonoPoly {
	pub voice_size: VoiceIndex,
	pub voice_mode: VoiceMode,
	pub voices: Vec<Voice>,
	pub last_voice_index: VoiceIndex,
}

impl MonoPoly {
	pub fn new(voice_size: u8) -> Self {
		Self {
			voice_size,
			voice_mode: VoiceMode::OpenPoly,
			voices: vec![Voice {note: 64, voice_index: 1}; voice_size as usize],
			last_voice_index: 0
		}
	}
	
	pub fn on_voice_on(&mut self, note: u8, velocity: u8, voice_params: &Vec<VoiceParams>) {
		let curr_index = self.last_voice_index as usize;
		let voice_params = &voice_params[curr_index];
		println!("{note} {velocity} {curr_index} {}", self.voices.len());
		self.voices[curr_index] = Voice {note, voice_index:  self.last_voice_index};

		voice_params.pitch.set_value(midi_hz(note as f64));
		voice_params.volume.set_value(velocity as f64 / 127.0);
		voice_params.pitch_bend.set_value(1.0);
		voice_params.control.set_value(1.0);

		self.last_voice_index = (self.last_voice_index + 1) % self.voice_size;
	}
	
	pub fn on_voice_off(&mut self, note: u8, voice_params: &Vec<VoiceParams>) {
			
		self.voices.iter().enumerate()
			.filter(|(i, voice)| voice.note == note)
			.for_each(|(i, voice)| {
				let voice_params = &voice_params[i];
				voice_params.control.set_value(-1.0);
			});
	}
}