use crate::ui::events::PressEvent;

pub struct Button {
    on_press: Box<dyn Fn(PressEvent)>,
    color: (),
}

pub fn button() {}
