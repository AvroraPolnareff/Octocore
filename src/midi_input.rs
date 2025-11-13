use crate::modulation::{ModDestination, ModDestinations};
use crate::param::Param;
use crate::synth_params::{OpParams, SynthParams};
use crate::ui_state::{InputEvent, OpPage, Page, UIState};
use anyhow::bail;
use fundsp::hacker::Atomic;
use fundsp::shared::Shared;
use fundsp::Float;
use midi_msg::{ChannelVoiceMsg, ControlChange, MidiMsg};
use midir::{Ignore, MidiInput, MidiInputPort};
use read_input::prelude::input;
use read_input::prelude::*;
use std::sync::mpsc::Sender;

pub fn encoder_to_value(input: u8, value: f64, intensity: f64) -> f64 {
    if input > 64 {
        value + (-128 + input as i8) as f64 / intensity
    } else {
        value + input as f64 / intensity
    }
}

pub fn encoder_to_shared<F: Float + Atomic>(input: u8, value: &Shared<F>, intensity: F) {
    value.set_value(F::from_f64(encoder_to_value(
        input,
        value.value().to_f64(),
        intensity.to_f64(),
    )))
}

pub fn encoder_to_param(input: u8, value: &Param, intensity: f64) {
    value.set_value(encoder_to_value(
        input,
        value.unmodulated_value(),
        intensity,
    ))
}

pub fn control_to_pages(control: ControlChange, ui: &UIState, in_tx: &Sender<InputEvent>) {
    let mut page = ui.page.lock().unwrap();
    let mut op_subpage = ui.op_subpage.lock().unwrap();
    match control {
        ControlChange::CC { control, value } => {
            if value > 0 {
                match control {
                    102 => {
                        *page = Page::Op(0);
                        in_tx.send(InputEvent::PageChange(Page::Op(0))).unwrap();
                    }
                    103 => {
                        *page = Page::Op(1);
                        in_tx.send(InputEvent::PageChange(Page::Op(1))).unwrap();
                    }
                    104 => {
                        *page = Page::Op(2);
                        in_tx.send(InputEvent::PageChange(Page::Op(2))).unwrap();
                    }
                    105 => {
                        *page = Page::Op(3);
                        in_tx.send(InputEvent::PageChange(Page::Op(3))).unwrap();
                    }
                    106 => {
                        *page = Page::Modulation;
                        in_tx
                            .send(InputEvent::PageChange(Page::Modulation))
                            .unwrap();
                    }
                    //105 => { *page = Page::Op4; ui_tx.send(InputEvent::PageChange(Page::Op4)).unwrap(); }
                    _ => {}
                }
            }
        }
        ControlChange::CCHighRes {
            control1,
            control2: _,
            value,
        } => {
            if value > 0 {
                match control1 {
                    20 => {
                        *op_subpage = OpPage::Tone;
                        in_tx
                            .send(InputEvent::OpSubpageChange(OpPage::Tone))
                            .unwrap();
                    }
                    21 => {
                        *op_subpage = OpPage::Amp;
                        in_tx
                            .send(InputEvent::OpSubpageChange(OpPage::Amp))
                            .unwrap();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub enum Pot {
    MainPot(u8, u8),
}

pub fn pots_to_sub_page(pot: &Pot, op_subpage: OpPage, op_params: &OpParams) {
    match op_subpage {
        OpPage::Tone => {
            if let Pot::MainPot(id, value) = pot {
                match id {
                    1 => encoder_to_param(*value, &op_params.volume, 512.),
                    2 => encoder_to_param(*value, &op_params.ratio, 1.),
                    _ => {}
                }
            }
        }
        OpPage::Amp => {
            if let Pot::MainPot(id, value) = pot {
                match id {
                    1 => encoder_to_shared(*value, &op_params.adsr_params.a, 32.),
                    2 => encoder_to_shared(*value, &op_params.adsr_params.d, 32.),
                    3 => encoder_to_shared(*value, &op_params.adsr_params.s, 32.),
                    4 => encoder_to_shared(*value, &op_params.adsr_params.r, 32.),
                    _ => {}
                }
            }
        }
    }
}

pub fn pots_to_controls<'a>(
    control: ControlChange,
    voice_params: &SynthParams,
    ui: &UIState,
    mod_dests: &ModDestinations,
    in_tx: &Sender<InputEvent>,
) {
    let page = ui.page.lock().unwrap();
    let op_subpage = ui.op_subpage.lock().unwrap();
    let mut dest = ui.lfo_dest.lock().unwrap();

    let pot = match control {
        ControlChange::SoundControl2(x) => Some(Pot::MainPot(1, x)),
        ControlChange::SoundControl3(x) => Some(Pot::MainPot(2, x)),
        ControlChange::SoundControl4(x) => Some(Pot::MainPot(3, x)),
        ControlChange::SoundControl5(x) => Some(Pot::MainPot(4, x)),
        ControlChange::SoundControl6(x) => Some(Pot::MainPot(5, x)),
        ControlChange::SoundControl7(x) => Some(Pot::MainPot(6, x)),
        ControlChange::SoundControl8(x) => Some(Pot::MainPot(7, x)),
        ControlChange::SoundControl9(x) => Some(Pot::MainPot(8, x)),
        _ => None,
    };

    if let Some(pot) = pot {
        match *page {
            Page::Op(x) => {
                pots_to_sub_page(&pot, op_subpage.to_owned(), &voice_params.ops[x as usize])
            }
            Page::Modulation => {
                if let Pot::MainPot(1, x) = pot {
                    let index = encoder_to_value(x, dest.to_owned().0 as f64, 1.).floor() as usize
                        % mod_dests.len();
                    *dest = mod_dests[index].clone();
                    in_tx
                        .send(InputEvent::LFO(dest.to_owned().1.clone()))
                        .unwrap();
                }
            }
            _ => {}
        }
    }
}

pub fn midi_to_params(
    midi_msg: MidiMsg,
    voice_params: &SynthParams,
    ui: &UIState,
    in_tx: &Sender<InputEvent>,
    mod_destinations: &ModDestinations,
) {
    match midi_msg {
        MidiMsg::ChannelVoice { channel, msg } => {
            println!("Received {channel} {msg:?}");
            match msg {
                ChannelVoiceMsg::NoteOn { note, velocity } => {
                    if note > 12 {
                        // filter encoder touches on push 2
                        in_tx.send(InputEvent::NoteOn { note, velocity }).unwrap()
                    }
                }
                ChannelVoiceMsg::NoteOff { note, velocity: _ } => {
                    in_tx.send(InputEvent::NoteOff { note }).unwrap()
                }
                // ChannelVoiceMsg::PitchBend { bend } => {
                // 	voice_params.pitch_bend.set_value(pitch_bend_factor(bend));
                // }
                ChannelVoiceMsg::ControlChange { control } => {
                    control_to_pages(control, ui, &in_tx);
                    pots_to_controls(control, voice_params, ui, mod_destinations, in_tx);
                    if let ControlChange::CC {
                        control: 52,
                        value: x,
                    } = control
                    {
                        // if x > 0 {
                        // 	in_tx.send(InputEvent::LFO(0.5)).unwrap()
                        // } else {
                        // 	in_tx.send(InputEvent::LFO(0.0)).unwrap()
                        // }
                    }
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
    voice_params: SynthParams,
    ui: UIState,
    in_tx: Sender<InputEvent>,
    mod_destinations: ModDestinations,
) -> anyhow::Result<()> {
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;
    let _conn_in = midi_in
        .connect(
            &in_port,
            "midir-read-input",
            move |_stamp, message, _| {
                let (msg, _len) = MidiMsg::from_midi(message).unwrap();
                midi_to_params(msg, &voice_params, &ui, &in_tx, &mod_destinations)
            },
            (),
        )
        .unwrap();
    println!("Connection open, reading input from '{in_port_name}'");

    let _ = input::<String>().msg("(press enter to exit)...\n").get();
    println!("Closing connection");
    Ok(())
}

pub fn get_midi_device(midi_in: &mut MidiInput) -> anyhow::Result<MidiInputPort> {
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    if in_ports.is_empty() {
        bail!("No MIDI devices attached")
    } else {
        let (port, _) = in_ports
            .iter()
            .map(|port| (port, midi_in.port_name(port).unwrap()))
            .find(|(_, name)| name.contains("Ableton Push 2"))
            .expect("Can't find Ableton Push 2 device");
        Ok(port.clone())
    }
}
