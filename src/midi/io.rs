use anyhow::bail;
use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputConnection, MidiOutputPort};
use read_input::prelude::*;
use std::sync::mpsc::Sender;

pub fn get_midi_out_device(midi_out: &mut MidiOutput) -> anyhow::Result<MidiOutputPort> {
    let out_ports = midi_out.ports();
    if out_ports.is_empty() {
        bail!("No MIDI devices attached")
    } else {
        let (port, _) = out_ports
            .iter()
            .map(|port| (port, midi_out.port_name(port).unwrap()))
            .find(|(_, name)| name.contains("Ableton Push 2"))
            .expect("Can't find Ableton Push 2 device");
        Ok(port.clone())
    }
}

pub fn get_midi_in_device(midi_in: &mut MidiInput) -> anyhow::Result<MidiInputPort> {
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

pub fn run_input(
    midi_in: MidiInput,
    in_port: MidiInputPort,
    in_tx: Sender<Vec<u8>>,
) -> anyhow::Result<()> {
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            let _ = in_tx.send(Vec::from(message));
        },
        (),
    );
    if let Err(e) = _conn_in {
        panic!("Cannot connect to the Midi Input {}", e.to_string())
    }
    println!("Connection open, reading input from '{in_port_name}'");

    let _ = input::<String>().msg("(press enter to exit)...\n").get();
    println!("Closing connection");
    Ok(())
}

pub fn get_midi_out_connection(
    midi_out: MidiOutput,
    out_port: &MidiOutputPort,
) -> MidiOutputConnection {
    let out_port_name = midi_out
        .port_name(out_port)
        .expect("Cannot get output name");
    println!("Connection open, output from '{out_port_name}'");
    midi_out
        .connect(out_port, "midir-write-output")
        .expect(&format!("Error while openening connection {out_port_name}"))
}
