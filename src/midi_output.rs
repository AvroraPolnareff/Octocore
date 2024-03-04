use std::sync::mpsc::Receiver;
use anyhow::bail;
use midi_msg::{Channel, ChannelVoiceMsg, ControlChange};
use midi_msg::MidiMsg;
use crate::ui_state::{OpPage, Page, UIEvent};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use read_input::prelude::*;

pub fn get_midi_out_device(midi_out: &mut MidiOutput) -> anyhow::Result<MidiOutputPort> {
	let out_ports = midi_out.ports();
	if out_ports.is_empty() {
		bail!("No MIDI devices attached")
	} else {
		for (i, port) in out_ports.iter().enumerate() {
			println!("{i}. {}", midi_out.port_name(port).unwrap())
		}
		let port = input::<usize>().msg("Select output port: ").get();
		println!(
			"Chose MIDI device {}",
			midi_out.port_name(&out_ports[port]).unwrap()
		);
		Ok(out_ports[port].clone())
	}
}

fn send_cc(conn: &mut MidiOutputConnection, control: ControlChange) {
	conn.send(&MidiMsg::to_midi(
		&MidiMsg::ChannelVoice {
			channel: Channel::Ch1,
			msg: ChannelVoiceMsg::ControlChange { control }
		}
	)).expect(&format!("Cannot send {control:?}"));
}

const FIRST_LEDS_ROW: [u8; 2] = [
	102, 103
	// , 104, 105, 106, 107, 108, 109
];
const SECOND_LEDS_ROW: [u8; 2] = [
	20, 21
	// , 22, 23, 24, 25, 26, 27
];

struct Led {
	led_num: u8,
	led_color: u8,
	neutral_color: u8,
}

fn send_switch<const T: usize>(
	led: Led,
	leds: [u8; T],
	conn: &mut MidiOutputConnection
) {
	for led_num in leds {
		if led_num == led.led_num {
			conn.send(&[0xB0, led_num, led.led_color]).unwrap()
		} else {
			conn.send(&[0xB0, led_num, led.neutral_color]).unwrap()
		}
	}
}

pub fn run_midi_out(
	midi_out: MidiOutput,
	out_port: &MidiOutputPort,
	rx: Receiver<UIEvent>
) {
	let out_port_name = midi_out.port_name(out_port).expect("Cannot get output name");
	let mut conn = midi_out.connect(out_port, "midir-write-output")
		.expect(&format!("Error while openening connection {out_port_name}"));
	send_switch(Led {led_num: 102, led_color: 122, neutral_color: 124}, FIRST_LEDS_ROW, &mut conn);
	send_switch(Led {led_num: 20, led_color: 122, neutral_color: 124}, SECOND_LEDS_ROW, &mut conn);
	std::thread::spawn(move || {
		for event in rx {
			match event {
				UIEvent::OpSubpageChange(page) => {
					match page {
						OpPage::Tone => {
							send_switch(Led {led_num: 102, led_color: 122, neutral_color: 124}, FIRST_LEDS_ROW, &mut conn)
						}
						OpPage::Amp => {
							send_switch(Led {led_num: 103, led_color: 122, neutral_color: 124}, FIRST_LEDS_ROW, &mut conn)
						}
					}
				}
				UIEvent::PageChange(page) => {
					match page {
						Page::Op1 => {
							send_switch(Led {led_num: 20, led_color: 122, neutral_color: 124}, SECOND_LEDS_ROW, &mut conn)
						}
						Page::Op2 => {
							send_switch(Led {led_num: 21, led_color: 122, neutral_color: 124}, SECOND_LEDS_ROW, &mut conn)
						}
						Page::Op3 => {
							send_switch(Led {led_num: 22, led_color: 122, neutral_color: 124}, SECOND_LEDS_ROW, &mut conn)
						}
						Page::Op4 => {
							send_switch(Led {led_num: 23, led_color: 122, neutral_color: 124}, SECOND_LEDS_ROW, &mut conn)
						}
					}
				}
				_ => {}
			}
		}
	});
	
}