extern crate octocore;

use std::{sync::mpsc::channel, time::Duration};

use midir::{MidiInput, MidiOutput};
use octocore::midi::{
    self,
    colors::{self, Color, ColorMessage, ColoredControl, LedAnimation},
    controls::{self, PushButton, PushPad},
    io, sysex,
};

fn main() -> anyhow::Result<()> {
    println!("OOOOOo");
    let mut midi_in = MidiInput::new("midir reading input")?;
    let mut midi_out = MidiOutput::new("midir writing output")?;
    let out_port = io::get_midi_out_device(&mut midi_out)?;
    let in_port = io::get_midi_in_device(&mut midi_in)?;
    let mut out_conn = io::get_midi_out_connection(midi_out, &out_port);
    let mut palette_color = sysex::PaletteColor {
        index: 22,
        red: 240,
        blue: 120,
        green: 0,
        white: 125,
    };
    let mut color = Color(22, LedAnimation::Pulsing(colors::AnimationSpeed::q2));
    let colored_control_msg = ColorMessage {
        color: color,
        control: ColoredControl::Button(PushButton::LowerRow(controls::TrackIndex::T1)),
    };
    let sysex = palette_color.to_sysex();
    out_conn.send(&sysex.0)?;
    println!("{:?}", sysex.0);
    out_conn.send(&colored_control_msg.to_midi())?;
    for x in (0..=63) {
        let colored_pad_msg = ColorMessage {
            color: color,
            control: ColoredControl::Pad(PushPad::new(x)),
        };
        out_conn.send(&colored_pad_msg.to_midi())?;
    }

    println!("{:?}", colored_control_msg.to_midi());
    out_conn.send(&[191, 119, 127])?;
    let (sender, receiver) = channel::<Vec<u8>>();
    std::thread::spawn(move || loop {
        out_conn.send(&[0xF8]).unwrap();
        std::thread::sleep(Duration::from_millis(20));
    });
    std::thread::spawn(move || loop {
        let msg = receiver.recv().unwrap();
        if !msg.starts_with(&[254]) {
            println!("{:?}", msg)
        }
    });
    io::run_input(midi_in, in_port, sender);

    anyhow::Ok(())
}
