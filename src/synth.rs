use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::audionode::{Pipe, Stack};
use fundsp::audiounit::AudioUnit64;
use fundsp::combinator::An;
use fundsp::envelope::EnvelopeIn;
use fundsp::hacker::{Frame, sine, var, Shared, NetBackend64, pass, sine_hz};
use fundsp::hacker32::Var;
use fundsp::prelude::U5;
use crate::adsr::adsr;
use crate::Poly::VoiceIndex;
use crate::voice_params::{AdsrParams, VoiceParams};


pub fn c_adsr(
	adsr_params: &AdsrParams,
	control: &Shared<f64>
) -> An<Pipe<
	f64, Stack<
		f64,
		Stack<f64, Stack<f64, Stack<f64, Var<f64>, Var<f64>>, Var<f64>>, Var<f64>>,
		Var<f64>
	>,
	EnvelopeIn<
		f64, f64, impl Fn(f64, &Frame<f64, U5>) -> f64 + Sized + Clone + Sized, U5, f64>
>> {
	(
		var(&adsr_params.a)
			| var(&adsr_params.d)
			| var(&adsr_params.s)
			| var(&adsr_params.r)
			| var(&control)
	)>> adsr()
}

pub fn create_sound(
	voice_params: &VoiceParams,
	voice_index: &VoiceIndex
) -> Box<dyn AudioUnit64> {

	let bf = || var(&voice_params.pitch) * var(&voice_params.pitch_bend);
	let modulator = bf() * var(&voice_params.op2.ratio)
		>> sine() * c_adsr(&voice_params.op2.adsr_params, &voice_params.control)
		* var(&voice_params.op2.volume);
	let bff = || bf() * var(&voice_params.op1.ratio);
	let base_tone =
		modulator * bff() + bff() >> sine() * var(&voice_params.op1.volume);

	Box::new(
		base_tone * c_adsr(&voice_params.op1.adsr_params, &voice_params.control) * var(&voice_params.volume)
	)
}

pub fn sine_lfo() -> Box<dyn AudioUnit64> {
	Box::new(
		pass() * (sine_hz(0.5))
	)
}

pub fn pitch_bend_factor(bend: u16) -> f64 {
	2.0_f64.powf(((bend as f64 - 8192.0) / 8192.0) / 12.0)
}

fn run_synth<T: SizedSample + FromSample<f64>>(
	device: Device,
	config: StreamConfig,
	backend: NetBackend64
) {
	std::thread::spawn(move || {
		let mut backend = backend;
		backend.set_sample_rate(config.sample_rate.0 as f64);

		let mut next_value = move || backend.get_stereo();
		let channels = config.channels as usize;
		let err_fn = |err| eprintln!("an error occurred on stream: {err}");
		let stream = device
			.build_output_stream(
				&config,
				move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
					write_data(data, channels, &mut next_value)
				},
				err_fn,
				None,
			)
			.unwrap();

		stream.play().unwrap();
		loop {
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});
}

pub fn run_output(
	backend: NetBackend64
) {
	let host = cpal::default_host();
	let device = host
		.default_output_device()
		.expect("failed to find a default output device");
	let config = device.default_output_config().unwrap();
	match config.sample_format() {
		SampleFormat::F32 => {
			run_synth::<f32>(device, config.into(), backend)
		}
		SampleFormat::I16 => {
			run_synth::<i16>(device, config.into(), backend)
		}
		SampleFormat::U16 => {
			run_synth::<u16>(device, config.into(), backend)
		}
		_ => panic!("Unsupported format"),
	}
}

pub fn write_data<T: SizedSample + FromSample<f64>>(
	output: &mut [T],
	channels: usize,
	next_sample: &mut dyn FnMut() -> (f64, f64),
) {
	for frame in output.chunks_mut(channels) {
		let sample = next_sample();
		let left: T = T::from_sample(sample.0);
		let right: T = T::from_sample(sample.1);

		for (channel, sample) in frame.iter_mut().enumerate() {
			*sample = if channel & 1 == 0 { left } else { right };
		}
	}
}

