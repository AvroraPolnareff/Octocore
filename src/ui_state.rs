use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum OpPage {
	Tone,
	Amp,
}

pub enum Page {
	Op1,
	Op2,
	Op3,
	Op4
}

#[derive(Clone)]
pub struct UIState {
	pub page: Arc<Mutex<Page>>,
	pub op_subpage: Arc<Mutex<OpPage>>
}

pub enum InputEvent {
	PageChange(Page),
	OpSubpageChange(OpPage),
	LFO(f64),
	NoteOn {note: u8, velocity: u8},
	NoteOff {note: u8}
}
