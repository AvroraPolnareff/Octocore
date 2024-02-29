use std::sync::Mutex;

pub enum OpPage {
	Tone,
	Amp,
}

pub enum Page {
	OpSettings(OpPage)
}

pub struct UIState {
	page: Mutex<Page>
}