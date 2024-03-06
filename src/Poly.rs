use fundsp::hacker32::midi_hz;

pub type VoiceIndex = u8;

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
			voices: Vec::new(),
			last_voice_index: voice_size
		}
	}
	
	pub fn on_voice_on(&mut self, note: u8, velocity: u8) {
		self.voices.insert(
			self.last_voice_index as usize,
			Voice {note, voice_index:  self.last_voice_index}
		);
		self.last_voice_index += 1;
	}
	
	pub fn on_voice_off(&mut self, note: u8) {
		
	}
}