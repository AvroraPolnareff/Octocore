use std::sync::{Arc, Mutex};
use crate::modulation::{ModDestination};

#[derive(Clone)]
pub enum OpPage {
	Tone,
	Amp,
}

pub enum Page {
	Op1,
	Op2,
	// Op3,
	// Op4,
	Modulation
}

#[derive(Clone)]
pub struct UIState {
	pub page: Arc<Mutex<Page>>,
	pub op_subpage: Arc<Mutex<OpPage>>,
	pub lfo_dest: Arc<Mutex<(usize, ModDestination)>>
}

pub enum InputEvent {
	PageChange(Page),
	OpSubpageChange(OpPage),
	LFO(ModDestination),
	NoteOn {note: u8, velocity: u8},
	NoteOff {note: u8}
}
