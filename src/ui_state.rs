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

pub enum UIEvent {
	PageChange(Page),
	OpSubpageChange(OpPage),
}
