use skia_safe::Canvas;

pub struct Color(u8);

pub struct PressEvent {
    velocity: u8,
}

pub struct Pad {
    pub color: Color,
    pub handle_press: Box<dyn Fn(PressEvent)>,
}

pub enum NoteEvent {
    On(u8),
    Off(u8),
}

impl Pad {
    pub fn note_in(&mut self, e: NoteEvent) {}
    // pub fn draw(&self, canvas: &Canvas) {

    // }
}
